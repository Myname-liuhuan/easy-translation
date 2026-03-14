import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

interface TranslationResult {
  text: string;
  from: string;
  to: string;
}

// Language display names
const languageNames: Record<string, string> = {
  zh: '中文',
  en: 'English',
  other: 'Auto',
};

export function useTranslation() {
  const input = ref('');
  const output = ref('');
  const loading = ref(false);
  const error = ref<string | null>(null);
  const fromLang = ref('');
  const toLang = ref('');

  const getLanguageName = (code: string): string => {
    return languageNames[code] || code.toUpperCase();
  };

  const translate = async (text: string) => {
    if (!text.trim()) {
      output.value = '';
      error.value = null;
      fromLang.value = '';
      toLang.value = '';
      return;
    }

    loading.value = true;
    error.value = null;

    try {
      const result = await invoke<TranslationResult>('translate_text', { text });
      output.value = result.text;
      fromLang.value = getLanguageName(result.from);
      toLang.value = getLanguageName(result.to);
    } catch (e) {
      error.value = String(e);
      output.value = '';
      fromLang.value = '';
      toLang.value = '';
    } finally {
      loading.value = false;
    }
  };

  return {
    input,
    output,
    loading,
    error,
    fromLang,
    toLang,
    translate,
  };
}
