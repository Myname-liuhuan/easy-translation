import { ref, onMounted, onUnmounted } from 'vue';
import { getCurrentWindow, type Theme as TauriTheme } from '@tauri-apps/api/window';

export type Theme = 'dark' | 'light';

export function useTheme() {
  const theme = ref<Theme>('dark');
  let mediaQuery: MediaQueryList | null = null;
  let unlistenThemeChanged: (() => void) | null = null;

  const applyTheme = (t: Theme) => {
    document.documentElement.setAttribute('data-theme', t);
  };

  const initTheme = async () => {
    try {
      // Tauri 2.0 API - 尝试获取系统主题
      const appWindow = getCurrentWindow();
      const systemTheme = await appWindow.theme();
      theme.value = systemTheme === 'light' ? 'light' : 'dark';
      applyTheme(theme.value);

      // 监听主题变化
      unlistenThemeChanged = await appWindow.onThemeChanged((event: { payload: TauriTheme }) => {
        const newTheme = event.payload === 'light' ? 'light' : 'dark';
        theme.value = newTheme;
        applyTheme(newTheme);
      });
    } catch {
      // Tauri API 不可用，回退到 media query
      mediaQuery = window.matchMedia('(prefers-color-scheme: light)');
      theme.value = mediaQuery.matches ? 'light' : 'dark';
      applyTheme(theme.value);

      mediaQuery.addEventListener('change', handleMediaQueryChange);
    }
  };

  const handleMediaQueryChange = (e: MediaQueryListEvent) => {
    theme.value = e.matches ? 'light' : 'dark';
    applyTheme(theme.value);
  };

  onMounted(() => {
    initTheme();
  });

  onUnmounted(() => {
    if (unlistenThemeChanged) {
      unlistenThemeChanged();
    }
    if (mediaQuery) {
      mediaQuery.removeEventListener('change', handleMediaQueryChange);
    }
  });

  return { theme };
}
