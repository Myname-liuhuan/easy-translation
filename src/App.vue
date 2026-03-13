<script setup lang="ts">
import { watch } from 'vue';
import { useTranslation } from './composables/useTranslation';
import { useDebounce } from './composables/useDebounce';
import { useWindowManager } from './composables/useWindowManager';

const { input, output, loading, error, translate } = useTranslation();
const debouncedInput = useDebounce(input, 300);

// Initialize window manager (global shortcut, blur listener, ESC listener)
useWindowManager();

// Watch debounced input and translate
watch(debouncedInput, (newValue) => {
  translate(newValue);
});
</script>

<template>
  <div class="app-container">
    <!-- Custom title bar with drag region -->
    <div class="title-bar" data-tauri-drag-region>
      <span class="title">Easy Translation</span>
      <span class="shortcut-hint">⌥T / Alt+T</span>
    </div>

    <!-- Main content -->
    <div class="content">
      <!-- Input area -->
      <div class="input-section">
        <textarea
          v-model="input"
          placeholder="Enter text to translate..."
          class="input-textarea"
          autofocus
        ></textarea>
      </div>

      <!-- Divider -->
      <div class="divider"></div>

      <!-- Output area -->
      <div class="output-section">
        <div v-if="loading" class="loading">Translating...</div>
        <div v-else-if="error" class="error">{{ error }}</div>
        <div v-else-if="output" class="output-text">{{ output }}</div>
        <div v-else class="placeholder">Translation will appear here</div>
      </div>
    </div>
  </div>
</template>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

:root {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', 'Oxygen',
    'Ubuntu', 'Cantarell', 'Fira Sans', 'Droid Sans', 'Helvetica Neue',
    sans-serif;
  font-size: 14px;
  line-height: 1.5;
  color: #e0e0e0;
  background-color: #1a1a1a;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

body {
  background-color: transparent;
  overflow: hidden;
}

.app-container {
  width: 100vw;
  height: 100vh;
  display: flex;
  flex-direction: column;
  background-color: #1a1a1a;
  border-radius: 8px;
  overflow: hidden;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);
}

.title-bar {
  height: 32px;
  background-color: #252525;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 12px;
  border-bottom: 1px solid #333;
  user-select: none;
}

.title {
  font-size: 12px;
  font-weight: 500;
  color: #999;
}

.shortcut-hint {
  font-size: 11px;
  color: #666;
}

.content {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.input-section {
  flex: 1;
  padding: 12px;
  display: flex;
  overflow: hidden;
}

.input-textarea {
  width: 100%;
  height: 100%;
  background-color: #1a1a1a;
  border: none;
  color: #e0e0e0;
  font-family: inherit;
  font-size: 14px;
  line-height: 1.6;
  resize: none;
  outline: none;
}

.input-textarea::placeholder {
  color: #555;
}

.divider {
  height: 1px;
  background-color: #333;
  margin: 0 12px;
}

.output-section {
  flex: 1;
  padding: 12px;
  overflow-y: auto;
}

.output-text {
  color: #e0e0e0;
  font-size: 14px;
  line-height: 1.6;
  white-space: pre-wrap;
  word-wrap: break-word;
}

.placeholder {
  color: #555;
  font-size: 14px;
}

.loading {
  color: #888;
  font-size: 14px;
}

.error {
  color: #ff6b6b;
  font-size: 14px;
}
</style>
