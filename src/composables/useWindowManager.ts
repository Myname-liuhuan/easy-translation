import { onMounted, onUnmounted } from 'vue';
import { register, unregister } from '@tauri-apps/plugin-global-shortcut';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { invoke } from '@tauri-apps/api/core';
import { listen, UnlistenFn } from '@tauri-apps/api/event';

const appWindow = getCurrentWindow();

export function useWindowManager() {
  let removeEscListener: (() => void) | undefined;
  let removeDockVisibilityListener: UnlistenFn | undefined;
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
      // Delay to allow window to render before switching to Regular mode (reduces flicker)
      await new Promise(resolve => setTimeout(resolve, 400));
      // Show in Dock when window is shown
      await invoke('set_dock_visibility', { visible: true });
      console.log('Window shown');
    } catch (error) {
      console.error('Failed to show window:', error);
    }
  };

  const hideWindow = async () => {
    try {
      console.log('Hiding window...');
      await appWindow.hide();
      // Don't hide from Dock - this is a "background" operation
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
      if (removeDockVisibilityListener) {
        removeDockVisibilityListener();
      }
    } catch (error) {
      console.error('Failed to cleanup:', error);
    }
  };

  onMounted(async () => {
    await setupGlobalShortcut();
    removeEscListener = setupEscListener();

    // Listen for dock visibility requests from Rust (tray icon click, window close)
    removeDockVisibilityListener = await listen<boolean>('request-dock-visibility', async (event) => {
      await invoke('set_dock_visibility', { visible: event.payload });
    });
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
