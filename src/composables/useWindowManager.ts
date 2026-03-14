import { onMounted, onUnmounted } from 'vue';
import { register, unregister } from '@tauri-apps/plugin-global-shortcut';
import { getCurrentWindow } from '@tauri-apps/api/window';

const appWindow = getCurrentWindow();

export function useWindowManager() {
  let removeEscListener: (() => void) | undefined;
  let lastToggleTime = 0;
  const TOGGLE_DEBOUNCE = 500; // ms

  const setupGlobalShortcut = async () => {
    try {
      await register('CommandOrControl+Alt+T', async () => {
        const now = Date.now();
        // Debounce: ignore if called within 500ms of last call
        if (now - lastToggleTime < TOGGLE_DEBOUNCE) {
          console.log('Shortcut debounced');
          return;
        }
        lastToggleTime = now;
        console.log('Shortcut triggered, toggling window...');
        await toggleWindow();
      });
    } catch (error) {
      console.error('Failed to register global shortcut:', error);
    }
  };

  const setupEscListener = () => {
    const handleKeyDown = async (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        await hideWindow();
      }
    };
    document.addEventListener('keydown', handleKeyDown);
    return () => document.removeEventListener('keydown', handleKeyDown);
  };

  const showWindow = async () => {
    try {
      console.log('Showing window...');
      await appWindow.show();
      await appWindow.setFocus();
      console.log('Window shown');
    } catch (error) {
      console.error('Failed to show window:', error);
    }
  };

  const hideWindow = async () => {
    try {
      console.log('Hiding window...');
      await appWindow.hide();
      console.log('Window hidden');
    } catch (error) {
      console.error('Failed to hide window:', error);
    }
  };

  const toggleWindow = async () => {
    try {
      const visible = await appWindow.isVisible();
      console.log('Window visible:', visible);
      if (visible) {
        await hideWindow();
      } else {
        await showWindow();
      }
    } catch (error) {
      console.error('Failed to toggle window:', error);
    }
  };

  const cleanup = async () => {
    try {
      await unregister('CommandOrControl+Alt+T');
      if (removeEscListener) {
        removeEscListener();
      }
    } catch (error) {
      console.error('Failed to cleanup:', error);
    }
  };

  onMounted(async () => {
    await setupGlobalShortcut();
    removeEscListener = setupEscListener();
  });

  onUnmounted(async () => {
    await cleanup();
  });

  return {
    showWindow,
    hideWindow,
    toggleWindow,
  };
}
