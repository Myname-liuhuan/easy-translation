<script setup lang="ts">
import { computed } from 'vue';
import type { WordInfo } from '../composables/useWordAnalysis';

const props = defineProps<{
  wordInfo: WordInfo;
  showTenses: boolean;
}>();

const emit = defineEmits<{
  (e: 'toggle-tenses'): void;
  (e: 'copy', form: string): void;
}>();

// POS color mapping
const posColor = computed(() => {
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
  return colorMap[props.wordInfo.pos?.toLowerCase() || ''] || '#9CA3AF';
});

// Copy to clipboard
const copyForm = async (form: string) => {
  try {
    await navigator.clipboard.writeText(form);
    emit('copy', form);
  } catch (e) {
    console.error('Copy failed:', e);
  }
};
</script>

<template>
  <div class="word-info">
    <!-- Main word info -->
    <div class="word-main">
      <span class="pos-indicator" :style="{ backgroundColor: posColor }"></span>
      <span class="pos-short">{{ wordInfo.pos || 'n.' }}</span>
      <span v-if="wordInfo.phonetic" class="phonetic">{{ wordInfo.phonetic }}</span>
      <span class="pos-chinese">{{ wordInfo.posChinese }}</span>

      <!-- Tenses toggle button -->
      <button
        v-if="wordInfo.tenses && wordInfo.tenses.length > 0"
        class="tenses-toggle"
        @click="emit('toggle-tenses')"
      >
        {{ showTenses ? '▲' : '▼' }} 时态 ({{ wordInfo.tenses.length }})
      </button>
    </div>

    <!-- Tenses list -->
    <Transition name="slide-fade">
      <div v-if="showTenses && wordInfo.tenses" class="tenses-list">
        <div
          v-for="(tense, index) in wordInfo.tenses"
          :key="index"
          class="tense-item"
          @click="copyForm(tense.form)"
        >
          <span class="tense-form">{{ tense.form }}</span>
          <span class="tense-desc">({{ tense.description }})</span>
        </div>
      </div>
    </Transition>
  </div>
</template>

<style scoped>
.word-info {
  padding: 8px 16px;
  background: var(--bg-tertiary);
  border-radius: 8px;
  animation: fadeIn 0.3s ease;
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

.word-main {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-wrap: wrap;
}

.pos-indicator {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  flex-shrink: 0;
}

.pos-short {
  font-size: 13px;
  font-weight: 600;
  color: var(--text-primary);
}

.phonetic {
  font-size: 13px;
  font-style: italic;
  color: var(--text-secondary);
}

.pos-chinese {
  font-size: 12px;
  color: var(--text-secondary);
}

.tenses-toggle {
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

.tenses-toggle:hover {
  background: var(--accent-primary);
  color: white;
  border-color: var(--accent-primary);
}

.tenses-list {
  margin-top: 10px;
  padding-top: 10px;
  border-top: 1px solid var(--border-color);
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.tense-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 6px 10px;
  background: var(--bg-secondary);
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.tense-item:hover {
  background: var(--accent-primary);
  transform: translateX(4px);
}

.tense-item:hover .tense-form,
.tense-item:hover .tense-desc {
  color: white;
}

.tense-form {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-primary);
}

.tense-desc {
  font-size: 11px;
  color: var(--text-secondary);
}

/* Transition animations */
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
</style>
