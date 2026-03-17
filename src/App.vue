<script setup lang="ts">
import { computed, watch, onMounted, onUnmounted } from 'vue';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { useTranslation } from './composables/useTranslation';
import { useDebounce } from './composables/useDebounce';
import { useWindowManager } from './composables/useWindowManager';
import { useTheme } from './composables/useTheme';

const { input, output, loading, error, fromLang, toLang, translate, clearData } = useTranslation();
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

// Watch debounced input and translate
watch(debouncedInput, (newValue) => {
  translate(newValue);
});
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
          <span v-else-if="output" class="output-text">{{ output }}</span>
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

.output-text {
  color: var(--text-primary);
  font-size: 15px;
  line-height: 1.7;
  white-space: pre-wrap;
  word-wrap: break-word;
  user-select: text;
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
