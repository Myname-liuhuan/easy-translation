import { ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';

// Dictionary entry type
export interface DictEntry {
  word: string;
  phonetic?: string;
  definition?: string;
  translation?: string;
  pos?: string;
  collins?: number;
  oxford?: number;
  tag?: string;
  bnc?: number;
  frq?: number;
  exchange?: string;
  detail?: string;
  audio?: string;
}

// Word info with parsed tenses
export interface WordInfo {
  word: string;
  phonetic?: string;
  pos?: string;
  posChinese?: string;
  translation?: string;
  collins?: number;
  tenses?: WordTense[];
}

// Extended translation result with parsed tenses
export interface TranslationResult extends DictEntry {
  tenses?: WordTense[];
  posChinese?: string;
}

// Combined result for translation
export interface TranslationResponse {
  text: string;
  from: string;
  to: string;
  dictResults?: TranslationResult[];
}

// Word tense information
export interface WordTense {
  form: string;
  description: string;
}

// Detection result
export interface DetectionResult {
  isEnglish: boolean;
  isChinese: boolean;
  isSingleWord: boolean;
}

// Language detection regex
const chineseRegex = /[\u4e00-\u9fff]/;
const englishWordRegex = /^[a-zA-Z]+$/;

export function useWordAnalysis() {
  const wordInfo = ref<WordInfo | null>(null);
  const translationResults = ref<TranslationResult[]>([]);
  const selectedTranslation = ref<string>('');
  const useLocalTranslation = ref(false);
  const loading = ref(false);
  const error = ref<string | null>(null);
  const showTenses = ref(false);

  // Detect input language and type
  const detectInput = (text: string): DetectionResult => {
    const trimmed = text.trim();
    return {
      isEnglish: englishWordRegex.test(trimmed),
      isChinese: chineseRegex.test(trimmed),
      isSingleWord: englishWordRegex.test(trimmed) && !trimmed.includes(' '),
    };
  };

  // Map POS to Chinese
  const posToChinese = (pos?: string): string => {
    if (!pos) return '';
    const posMap: Record<string, string> = {
      'n': '名词',
      'noun': '名词',
      'v': '动词',
      'verb': '动词',
      'adj': '形容词',
      'adjective': '形容词',
      'adv': '副词',
      'adverb': '副词',
      'prep': '介词',
      'preposition': '介词',
      'conj': '连词',
      'conjunction': '连词',
      'pron': '代词',
      'pronoun': '代词',
      'num': '数词',
      'numeral': '数词',
      'interj': '感叹词',
      'interjection': '感叹词',
    };
    return posMap[pos.toLowerCase()] || pos;
  };

  // Parse exchange field for tenses
  const parseTenses = (exchange?: string): WordTense[] => {
    if (!exchange) return [];

    const tenses: WordTense[] = [];

    try {
      // Try to parse as JSON first
      const parsed = JSON.parse(exchange);

      if (typeof parsed === 'object') {
        // Handle different exchange formats
        if (parsed.past) {
          tenses.push({ form: parsed.past, description: '过去式' });
        }
        if (parsed.pastParticiple) {
          tenses.push({ form: parsed.pastParticiple, description: '过去分词' });
        }
        if (parsed.presentParticiple) {
          tenses.push({ form: parsed.presentParticiple, description: '现在分词' });
        }
        if (parsed.thirdPerson) {
          tenses.push({ form: parsed.thirdPerson, description: '第三人称单数' });
        }
      }
    } catch {
      // Try to parse as string format like "p:ran;pp:run"
      const parts = exchange.split(';');
      for (const part of parts) {
        const [key, value] = part.split(':');
        if (key && value) {
          switch (key.trim()) {
            case 'p':
              tenses.push({ form: value.trim(), description: '过去式' });
              break;
            case 'pp':
              tenses.push({ form: value.trim(), description: '过去分词' });
              break;
            case 'ing':
              tenses.push({ form: value.trim(), description: '现在分词' });
              break;
            case '3':
              tenses.push({ form: value.trim(), description: '第三人称单数' });
              break;
          }
        }
      }
    }

    return tenses;
  };

  // Query local dictionary for English word
  const queryLocalDict = async (word: string): Promise<DictEntry | null> => {
    try {
      const result = await invoke<DictEntry | null>('query_dict', { word });
      return result;
    } catch (e) {
      console.error('Local dict query error:', e);
      return null;
    }
  };

  // Query local dictionary by translation (Chinese to English)
  const queryLocalDictByTranslation = async (translation: string): Promise<DictEntry[]> => {
    try {
      const results = await invoke<DictEntry[]>('query_dict_by_translation', { translation });
      return results;
    } catch (e) {
      console.error('Local dict query error:', e);
      return [];
    }
  };

  // Save word to local dictionary (for caching API results)
  const saveToLocalDict = async (entry: DictEntry): Promise<void> => {
    try {
      await invoke('save_dict_entry', { entry });
    } catch (e) {
      console.error('Save to local dict error:', e);
    }
  };

  // Analyze input text
  const analyzeInput = async (text: string) => {
    const trimmed = text.trim();
    if (!trimmed) {
      clearResults();
      return;
    }

    const detection = detectInput(trimmed);

    // Clear previous results
    wordInfo.value = null;
    translationResults.value = [];
    selectedTranslation.value = '';
    error.value = null;

    if (detection.isSingleWord) {
      // English word: query for word info and tenses
      await analyzeEnglishWord(trimmed);
    } else if (detection.isChinese) {
      // Chinese: query for multiple translations
      await analyzeChinese(trimmed);
    }
  };

  // Analyze English word
  const analyzeEnglishWord = async (word: string) => {
    loading.value = true;

    try {
      // Query local dictionary first
      const entry = await queryLocalDict(word.toLowerCase());

      if (entry) {
        const tenses = parseTenses(entry.exchange);
        wordInfo.value = {
          word: entry.word,
          phonetic: entry.phonetic,
          pos: entry.pos,
          posChinese: posToChinese(entry.pos),
          translation: entry.translation,
          collins: entry.collins,
          tenses: tenses.length > 0 ? tenses : undefined,
        };
      } else {
        // Try external API as fallback (placeholder)
        // In a real implementation, we would call Free Dictionary API here
        wordInfo.value = null;
      }
    } catch (e) {
      error.value = String(e);
    } finally {
      loading.value = false;
    }
  };

  // Analyze Chinese input - query local dictionary for translations
  const analyzeChinese = async (text: string) => {
    loading.value = true;
    useLocalTranslation.value = false;

    try {
      // Query local dictionary by translation
      const results = await queryLocalDictByTranslation(text);

      if (results.length > 0) {
        // Parse tenses for each result
        const resultsWithTenses: TranslationResult[] = results.map(result => ({
          ...result,
          tenses: parseTenses(result.exchange),
          posChinese: posToChinese(result.pos),
        }));
        translationResults.value = resultsWithTenses;
        
        // For single word translation, also set wordInfo for output display
        if (results.length === 1) {
          const firstResult = results[0];
          const tenses = parseTenses(firstResult.exchange);
          wordInfo.value = {
            word: firstResult.word,
            phonetic: firstResult.phonetic,
            pos: firstResult.pos,
            posChinese: posToChinese(firstResult.pos),
            translation: firstResult.translation,
            collins: firstResult.collins,
            tenses: tenses.length > 0 ? tenses : undefined,
          };
        }
        
        // Auto-select first result
        selectedTranslation.value = results[0].word;
        // Mark that we're using local translation
        useLocalTranslation.value = true;
      } else {
        // No local results, need to use external API
        translationResults.value = [];
        selectedTranslation.value = '';
        useLocalTranslation.value = false;
      }
    } catch (e) {
      error.value = String(e);
      useLocalTranslation.value = false;
    } finally {
      loading.value = false;
    }
  };

  // Check if should use local translation (no external API needed)
  const shouldUseLocalTranslation = computed(() => useLocalTranslation.value);

  // Select a translation
  const selectTranslation = (word: string) => {
    selectedTranslation.value = word;
  };

  // Clear all results
  const clearResults = () => {
    wordInfo.value = null;
    translationResults.value = [];
    selectedTranslation.value = '';
    error.value = null;
    showTenses.value = false;
  };

  // Toggle tenses display
  const toggleTenses = () => {
    showTenses.value = !showTenses.value;
  };

  return {
    wordInfo,
    translationResults,
    selectedTranslation,
    useLocalTranslation,
    shouldUseLocalTranslation,
    loading,
    error,
    showTenses,
    analyzeInput,
    selectTranslation,
    clearResults,
    toggleTenses,
    saveToLocalDict,
  };
}
