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

  // 请求版本号，用于确保只有最新请求的结果才会被应用
  let requestId = 0;

  const getLanguageName = (code: string): string => {
    return languageNames[code] || code.toUpperCase();
  };

  const translate = async (text: string) => {
    if (!text.trim()) {
      output.value = '';
      error.value = null;
      fromLang.value = '';
      toLang.value = '';
      requestId++; // 增加版本号，使之前的请求结果失效
      return;
    }

    loading.value = true;
    error.value = null;

    // 记录当前请求的版本号
    const currentRequestId = ++requestId;

    try {
      const result = await invoke<TranslationResult>('translate_text', { text });

      // 只有当这是最新请求时才应用结果
      if (currentRequestId === requestId) {
        output.value = result.text;
        fromLang.value = getLanguageName(result.from);
        toLang.value = getLanguageName(result.to);
      }
    } catch (e) {
      // 只有当这是最新请求时才应用错误
      if (currentRequestId === requestId) {
        error.value = String(e);
        output.value = '';
        fromLang.value = '';
        toLang.value = '';
      }
    } finally {
      // 只有当这是最新请求时才更新 loading 状态
      if (currentRequestId === requestId) {
        loading.value = false;
      }
    }
  };

  const clearData = () => {
    input.value = '';
    output.value = '';
    error.value = null;
    fromLang.value = '';
    toLang.value = '';
    loading.value = false;
    requestId++; // 增加版本号，使之前的请求结果失效
  };

  return {
    input,
    output,
    loading,
    error,
    fromLang,
    toLang,
    translate,
    clearData,
  };
}
