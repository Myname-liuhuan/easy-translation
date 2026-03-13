import { ref, watch, onUnmounted, type Ref } from 'vue';

export function useDebounce<T>(value: Ref<T>, delay: number = 300) {
  const debouncedValue = ref(value.value) as Ref<T>;
  let timeout: ReturnType<typeof setTimeout> | undefined;

  watch(value, (newValue) => {
    if (timeout) {
      clearTimeout(timeout);
    }
    timeout = setTimeout(() => {
      debouncedValue.value = newValue;
    }, delay);
  });

  onUnmounted(() => {
    if (timeout) {
      clearTimeout(timeout);
    }
  });

  return debouncedValue;
}
