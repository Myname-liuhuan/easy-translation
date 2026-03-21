<script setup lang="ts">
import { computed, watch, onMounted, onUnmounted } from 'vue';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';
import { useTranslation } from './composables/useTranslation';
import { useWordAnalysis } from './composables/useWordAnalysis';
import { useDebounce } from './composables/useDebounce';
import { useWindowManager } from './composables/useWindowManager';
import { useTheme } from './composables/useTheme';
import WordInfo from './components/WordInfo.vue';
import TranslationResults from './components/TranslationResults.vue';

// Dictionary entry type for invoke result
interface DictEntry {
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

const { input, output, loading, error, fromLang, toLang, translate, clearData } = useTranslation();
const {
  wordInfo,
  outputWordInfo,
  translationResults,
  selectedTranslation,
  useLocalTranslation,
  showTenses,
  showOutputTenses,
  analyzeInput,
  selectTranslation,
  clearResults,
  toggleTenses,
  toggleOutputTenses,
} = useWordAnalysis();
const debouncedInput = useDebounce(input, 300);

// Initialize theme (system light/dark mode support)
useTheme();

// Initialize window manager (global shortcut, blur listener, ESC listener)
useWindowManager();

// Listen for clear-data event from backend
let unlistenClearData: UnlistenFn | undefined;

onMounted(async () => {
  unlistenClearData = await listen('clear-data', () => {
    clearData();
    clearResults();
  });
});

onUnmounted(() => {
  if (unlistenClearData) {
    unlistenClearData();
  }
});

// Detect platform and show appropriate shortcut
const isMac = computed(() => {
  return navigator.platform.toLowerCase().includes('mac') ||
         navigator.userAgent.toLowerCase().includes('mac');
});

const shortcutHint = computed(() => {
  if (isMac.value) {
    return '⌘ + ⌥ + T';
  }
  return 'Ctrl + Alt + T';
});

// Check if we should show word analysis
const showWordInfo = computed(() => wordInfo.value !== null);
const showTranslationResults = computed(() => translationResults.value.length > 0);

// Watch debounced input - first analyze local dictionary, then decide on external API
watch(debouncedInput, async (newValue) => {
  if (!newValue) {
    translate('');
    analyzeInput(newValue);
    outputWordInfo.value = null; // 清空输出域的单词信息
    return;
  }

  // First analyze input for local dictionary
  await analyzeInput(newValue);

  // If local dictionary has results, use them; otherwise call external API
  if (useLocalTranslation.value && translationResults.value.length > 0) {
    output.value = translationResults.value[0]?.word || '';
  } else {
    // Use external API for translation
    await translate(newValue);
    
    // If translation result is a single English word, query dict for phonetic/tenses
    const translatedText = output.value;
    if (translatedText && /^[a-zA-Z]+$/.test(translatedText.trim())) {
      const word = translatedText.trim().toLowerCase();
      try {
        const entry = await invoke<DictEntry | null>('query_dict', { word });
        if (entry) {
          // Parse tenses from exchange field
          const parseTenses = (exchange?: string) => {
            if (!exchange) return [];
            const tenses: { form: string; description: string }[] = [];
            try {
              const parsed = JSON.parse(exchange);
              if (parsed.past) tenses.push({ form: parsed.past, description: '过去式' });
              if (parsed.pastParticiple) tenses.push({ form: parsed.pastParticiple, description: '过去分词' });
              if (parsed.presentParticiple) tenses.push({ form: parsed.presentParticiple, description: '现在分词' });
              if (parsed.thirdPerson) tenses.push({ form: parsed.thirdPerson, description: '第三人称单数' });
            } catch {
              const parts = exchange.split(';');
              for (const part of parts) {
                const [key, value] = part.split(':');
                if (key && value) {
                  switch (key.trim()) {
                    case 'p': tenses.push({ form: value.trim(), description: '过去式' }); break;
                    case 'pp': tenses.push({ form: value.trim(), description: '过去分词' }); break;
                    case 'ing': tenses.push({ form: value.trim(), description: '现在分词' }); break;
                    case '3': tenses.push({ form: value.trim(), description: '第三人称单数' }); break;
                  }
                }
              }
            }
            return tenses;
          };
          
          const posMap: Record<string, string> = {
            'n': '名词', 'noun': '名词', 'v': '动词', 'verb': '动词',
            'adj': '形容词', 'adjective': '形容词', 'adv': '副词', 'adverb': '副词',
          };
          
          const tenses = parseTenses(entry.exchange);
          outputWordInfo.value = {
            word: entry.word,
            phonetic: entry.phonetic,
            pos: entry.pos,
            posChinese: posMap[entry.pos?.toLowerCase() || ''] || entry.pos || '',
            translation: entry.translation,
            collins: entry.collins,
            tenses: tenses.length > 0 ? tenses : undefined,
          };
        }
      } catch (e) {
        console.error('Query dict error:', e);
      }
    }
  }
});

// Handle translation selection - update outputWordInfo when selection changes
watch(selectedTranslation, (newValue) => {
  if (newValue) {
    output.value = newValue;
    // Update outputWordInfo based on selected translation
    const selectedResult = translationResults.value.find(r => r.word === newValue);
    if (selectedResult) {
      outputWordInfo.value = {
        word: selectedResult.word,
        phonetic: selectedResult.phonetic,
        pos: selectedResult.pos,
        posChinese: selectedResult.posChinese,
        translation: selectedResult.translation,
        collins: selectedResult.collins,
        tenses: selectedResult.tenses,
      };
    }
  }
});

// Handle tense copy
const handleTenseCopy = (form: string) => {
  console.log('Copied:', form);
};

// Handle result selection
const handleSelect = (word: string) => {
  selectTranslation(word);
};

// POS color mapping for output area
const getPosColor = (pos?: string): string => {
  const colorMap: Record<string, string> = {
    'n': '#8B5CF6',
    'noun': '#8B5CF6',
    'v': '#3B82F6',
    'verb': '#3B82F6',
    'adj': '#10B981',
    'adjective': '#10B981',
    'adv': '#F59E0B',
    'adverb': '#F59E0B',
    'prep': '#6B7280',
    'preposition': '#6B7280',
    'conj': '#06B6D4',
    'conjunction': '#06B6D4',
    'pron': '#EC4899',
    'pronoun': '#EC4899',
  };
  return colorMap[pos?.toLowerCase() || ''] || '#9CA3AF';
};
</script>

<template>
  <div class="app-container">
    <!-- Main content -->
    <div class="content">
      <!-- Input area -->
      <div class="input-section">
        <div class="section-header">
          <span v-if="fromLang" class="lang-tag">{{ fromLang }}</span>
        </div>
        <textarea
          v-model="input"
          placeholder="输入要翻译的文本..."
          class="input-textarea"
          autofocus
        ></textarea>
      </div>

      <!-- Word Analysis Section -->
      <div v-if="showWordInfo || showTranslationResults" class="analysis-section">
        <!-- English word info with tenses -->
        <WordInfo
          v-if="showWordInfo"
          :word-info="wordInfo!"
          :show-tenses="showTenses"
          @toggle-tenses="toggleTenses"
          @copy="handleTenseCopy"
        />

        <!-- Chinese translation results -->
        <TranslationResults
          v-if="showTranslationResults"
          :results="translationResults"
          :selected-word="selectedTranslation"
          @select="handleSelect"
        />
      </div>

      <!-- Animated divider -->
      <div class="divider">
        <div class="divider-line"></div>
        <div class="divider-glow"></div>
      </div>

      <!-- Output area -->
      <div class="output-section">
        <div class="section-header">
          <span v-if="toLang" class="lang-tag">{{ toLang }}</span>
        </div>
        <div class="output-content">
          <span v-if="loading" class="loading">
            <span class="loading-dot"></span>
            翻译中...
          </span>
          <span v-else-if="error" class="error">{{ error }}</span>
          <template v-else-if="output">
            <div class="output-main">
              <span class="output-text">{{ output }}</span>
            </div>
            <!-- POS and tenses section for translated English word (Chinese to English only) -->
            <div v-if="outputWordInfo && toLang === 'English'" class="output-word-info">
              <div class="word-main">
                <span class="pos-indicator" :style="{ backgroundColor: getPosColor(outputWordInfo.pos) }"></span>
                <span class="pos-short">{{ outputWordInfo.pos || 'n.' }}</span>
                <span v-if="outputWordInfo.phonetic" class="phonetic">{{ outputWordInfo.phonetic }}</span>
                <span class="pos-chinese">{{ outputWordInfo.posChinese }}</span>
                <button
                  v-if="outputWordInfo.tenses && outputWordInfo.tenses.length > 0"
                  class="tenses-toggle"
                  @click="toggleOutputTenses"
                >
                  {{ showOutputTenses ? '▲' : '▼' }} 时态 ({{ outputWordInfo.tenses.length }})
                </button>
              </div>
              <Transition name="slide-fade">
                <div v-if="showOutputTenses && outputWordInfo.tenses" class="output-tenses">
                  <div
                    v-for="(tense, index) in outputWordInfo.tenses"
                    :key="index"
                    class="tense-item"
                    @click="handleTenseCopy(tense.form)"
                  >
                    <span class="tense-form">{{ tense.form }}</span>
                    <span class="tense-desc">({{ tense.description }})</span>
                  </div>
                </div>
              </Transition>
            </div>
          </template>
          <span v-else class="placeholder">翻译结果将显示在这里</span>
        </div>
      </div>
    </div>

    <!-- Footer status bar -->
    <div class="footer-bar">
      <span class="shortcut-hint">{{ shortcutHint }} 快速唤起</span>
    </div>
  </div>
</template>

<style>
/* Import theme variables */
@import './assets/theme.css';

/* Import elegant Chinese-friendly font */
@import url('https://fonts.googleapis.com/css2?family=Noto+Sans+SC:wght@300;400;500&display=swap');

* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html, body, #app {
  height: 100%;
  overflow: hidden;
}

.app-container {
  height: 100%;
  background: linear-gradient(145deg, var(--bg-primary) 0%, var(--bg-secondary) 50%, var(--bg-tertiary) 100%);
  display: flex;
  flex-direction: column;
  font-family: 'Noto Sans SC', -apple-system, BlinkMacSystemFont, sans-serif;
  color: var(--text-primary);
  position: relative;
  overflow: hidden;
}

/* Subtle noise texture overlay */
.app-container::before {
  content: '';
  position: absolute;
  inset: 0;
  background-image: url("data:image/svg+xml,%3Csvg viewBox='0 0 256 256' xmlns='http://www.w3.org/2000/svg'%3E%3Cfilter id='noise'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='0.9' numOctaves='4' stitchTiles='stitch'/%3E%3C/filter%3E%3Crect width='100%25' height='100%25' filter='url(%23noise)'/%3E%3C/svg%3E");
  opacity: var(--noise-opacity);
  pointer-events: none;
  z-index: 0;
}

.content {
  flex: 1;
  display: flex;
  flex-direction: column;
  position: relative;
  z-index: 1;
  min-height: 0;
}

.input-section,
.output-section {
  flex: 1;
  display: flex;
  flex-direction: column;
  position: relative;
  min-height: 0;
}

.section-header {
  height: 28px;
  padding: 0 16px;
  display: flex;
  align-items: center;
  flex-shrink: 0;
}

.lang-tag {
  font-size: 10px;
  font-weight: 500;
  color: var(--text-secondary);
  background: var(--accent-gradient);
  padding: 3px 10px;
  border-radius: 20px;
  border: 1px solid var(--border-tag);
  letter-spacing: 0.5px;
  text-transform: uppercase;
  transition: all 0.3s ease;
}

.lang-tag:hover {
  background: var(--accent-gradient-hover);
  border-color: var(--border-tag-hover);
}

.input-textarea {
  flex: 1;
  background: transparent;
  border: none;
  padding: 0 16px 12px;
  font-family: inherit;
  font-size: 15px;
  line-height: 1.7;
  resize: none;
  outline: none;
  color: var(--text-primary);
  min-height: 0;
}

.input-textarea::placeholder {
  color: var(--text-placeholder);
  transition: color 0.3s ease;
}

.input-textarea:focus::placeholder {
  color: var(--text-placeholder-focus);
}

/* Animated divider with glow effect */
.divider {
  position: relative;
  height: 2px;
  flex-shrink: 0;
  padding: 0 24px;
}

.divider-line {
  position: absolute;
  left: 24px;
  right: 24px;
  top: 50%;
  height: 1px;
  background: var(--divider-line);
}

.divider-glow {
  position: absolute;
  left: 50%;
  top: 50%;
  transform: translate(-50%, -50%);
  width: 120px;
  height: 6px;
  background: var(--divider-glow);
  animation: pulse 3s ease-in-out infinite;
}

@keyframes pulse {
  0%, 100% {
    opacity: 0.6;
    transform: translate(-50%, -50%) scaleX(1);
  }
  50% {
    opacity: 1;
    transform: translate(-50%, -50%) scaleX(1.3);
  }
}

.output-section {
  padding-bottom: 0;
}

/* Word Analysis Section */
.analysis-section {
  padding: 0 16px 8px;
  display: flex;
  flex-direction: column;
  gap: 8px;
  flex-shrink: 0;
}

.footer-bar {
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--bg-footer);
  border-top: 1px solid var(--border-color);
  position: relative;
  z-index: 1;
  flex-shrink: 0;
}

.shortcut-hint {
  font-size: 11px;
  color: var(--text-secondary);
  font-family: 'SF Mono', Monaco, 'Noto Sans SC', monospace;
  letter-spacing: 0.3px;
}

.output-content {
  flex: 1;
  padding: 0 16px 12px;
  overflow-y: auto;
  min-height: 0;
}

.output-main {
  display: flex;
  align-items: baseline;
  gap: 8px;
  flex-wrap: wrap;
}

.output-text {
  color: var(--text-primary);
  font-size: 15px;
  line-height: 1.7;
  white-space: pre-wrap;
  word-wrap: break-word;
  user-select: text;
}

.output-word-info {
  margin-top: 10px;
  padding: 8px 16px;
  background: var(--bg-tertiary);
  border-radius: 8px;
  animation: fadeIn 0.3s ease;
}

.output-word-info .word-main {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-wrap: wrap;
}

.output-word-info .pos-indicator {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  flex-shrink: 0;
}

.output-word-info .pos-short {
  font-size: 13px;
  font-weight: 600;
  color: var(--text-primary);
}

.output-word-info .phonetic {
  font-size: 13px;
  font-style: italic;
  color: var(--text-secondary);
}

.output-word-info .pos-chinese {
  font-size: 12px;
  color: var(--text-secondary);
}

.output-word-info .tenses-toggle {
  margin-left: auto;
  padding: 4px 10px;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 12px;
  color: var(--text-secondary);
  font-size: 11px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.output-word-info .tenses-toggle:hover {
  background: var(--accent-primary);
  color: white;
  border-color: var(--accent-primary);
}

.output-tenses {
  margin-top: 8px;
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
}

.tense-item {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  padding: 4px 8px;
  background: var(--bg-secondary);
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.tense-item:hover {
  background: var(--accent-primary);
}

.tense-item:hover .tense-form,
.tense-item:hover .tense-desc {
  color: white;
}

.tense-form {
  font-size: 12px;
  font-weight: 500;
  color: var(--text-primary);
}

.tense-desc {
  font-size: 10px;
  color: var(--text-secondary);
}

.placeholder {
  color: var(--text-placeholder);
  font-size: 14px;
  font-style: italic;
}

.loading {
  color: var(--text-secondary);
  font-size: 14px;
  display: inline-flex;
  align-items: center;
  gap: 8px;
}

.loading-dot {
  width: 6px;
  height: 6px;
  background: var(--accent-primary);
  border-radius: 50%;
  animation: bounce 1.4s ease-in-out infinite;
}

@keyframes bounce {
  0%, 80%, 100% {
    transform: scale(0.6);
    opacity: 0.4;
  }
  40% {
    transform: scale(1);
    opacity: 1;
  }
}

.error {
  color: var(--error);
  font-size: 14px;
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(-8px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

/* Transition animations for tenses */
.slide-fade-enter-active {
  transition: all 0.3s ease-out;
}

.slide-fade-leave-active {
  transition: all 0.2s ease-in;
}

.slide-fade-enter-from,
.slide-fade-leave-to {
  transform: translateY(-10px);
  opacity: 0;
}

/* Custom scrollbar */
.output-content::-webkit-scrollbar {
  width: 4px;
}

.output-content::-webkit-scrollbar-track {
  background: transparent;
}

.output-content::-webkit-scrollbar-thumb {
  background: var(--scrollbar-thumb);
  border-radius: 2px;
}

.output-content::-webkit-scrollbar-thumb:hover {
  background: var(--scrollbar-thumb-hover);
}
</style>
