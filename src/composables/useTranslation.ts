import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

export function useTranslation() {
  const input = ref('');
  const output = ref('');
  const loading = ref(false);
  const error = ref<string | null>(null);

  const translate = async (text: string) => {
    if (!text.trim()) {
      output.value = '';
      error.value = null;
      return;
    }

    loading.value = true;
    error.value = null;

    try {
      const result = await invoke<string>('translate_text', { text });
      output.value = result;
    } catch (e) {
      error.value = String(e);
      output.value = '';
    } finally {
      loading.value = false;
    }
  };

  return {
    input,
    output,
    loading,
    error,
    translate,
  };
}
