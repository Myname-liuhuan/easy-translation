<script setup lang="ts">
import { ref } from 'vue';
import type { TranslationResult } from '../composables/useWordAnalysis';

defineProps<{
  results: TranslationResult[];
  selectedWord: string;
}>();

const emit = defineEmits<{
  (e: 'select', word: string): void;
}>();

// Track which items are expanded
const expandedItems = ref<Set<string>>(new Set());

// Toggle expansion
const toggleExpand = (word: string) => {
  if (expandedItems.value.has(word)) {
    expandedItems.value.delete(word);
  } else {
    expandedItems.value.add(word);
  }
};

// Check if expanded
const isExpanded = (word: string): boolean => {
  return expandedItems.value.has(word);
};

// POS color mapping
const getPosColor = (pos?: string): string => {
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
  return colorMap[pos?.toLowerCase() || ''] || '#9CA3AF';
};

// Format Collins stars
const formatStars = (collins?: number): string => {
  if (!collins) return '';
  return '●'.repeat(collins) + '○'.repeat(5 - collins);
};

// Copy to clipboard
const copyForm = async (form: string) => {
  try {
    await navigator.clipboard.writeText(form);
  } catch (e) {
    console.error('Copy failed:', e);
  }
};
</script>

<template>
  <div class="translation-results">
    <!-- Header -->
    <div class="results-header">
      <span class="results-count">共 {{ results.length }} 个结果</span>
    </div>

    <!-- Results list -->
    <div class="results-list">
      <div
        v-for="(result, index) in results"
        :key="index"
        class="result-item"
        :class="{ selected: selectedWord === result.word }"
      >
        <!-- Main row -->
        <div class="result-main" @click="emit('select', result.word)">
          <span class="pos-indicator" :style="{ backgroundColor: getPosColor(result.pos) }"></span>
          <span class="word">{{ result.word }}</span>
          <span v-if="result.phonetic" class="phonetic">{{ result.phonetic }}</span>
          <span class="pos-label">{{ result.posChinese }}</span>
          <span v-if="result.collins" class="stars">{{ formatStars(result.collins) }}</span>

          <!-- Tenses toggle button -->
          <button
            v-if="result.tenses && result.tenses.length > 0"
            class="tenses-toggle"
            :class="{ expanded: isExpanded(result.word) }"
            @click.stop="toggleExpand(result.word)"
          >
            {{ isExpanded(result.word) ? '▲' : '▼' }} 时态
          </button>
        </div>

        <!-- Tenses section -->
        <Transition name="slide-fade">
          <div v-if="result.tenses && result.tenses.length > 0 && isExpanded(result.word)" class="tenses-section">
            <div
              v-for="(tense, tIndex) in result.tenses"
              :key="tIndex"
              class="tense-item"
              @click="copyForm(tense.form)"
            >
              <span class="tense-form">{{ tense.form }}</span>
              <span class="tense-desc">({{ tense.description }})</span>
            </div>
          </div>
        </Transition>
      </div>
    </div>
  </div>
</template>

<style scoped>
.translation-results {
  display: flex;
  flex-direction: column;
  max-height: 240px;
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

.results-header {
  padding: 8px 16px;
  border-bottom: 1px solid var(--border-color);
  flex-shrink: 0;
}

.results-count {
  font-size: 12px;
  color: var(--text-secondary);
}

.results-list {
  flex: 1;
  overflow-y: auto;
  padding: 4px 0;
}

.result-item {
  border-bottom: 1px solid var(--border-subtle);
}

.result-item:last-child {
  border-bottom: none;
}

.result-main {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 16px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.result-main:hover {
  background: var(--bg-secondary);
}

.result-item.selected .result-main {
  background: linear-gradient(135deg, rgba(59, 130, 246, 0.15), rgba(139, 92, 246, 0.15));
  border-left: 3px solid var(--accent-primary);
}

.pos-indicator {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  flex-shrink: 0;
}

.word {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
  min-width: 70px;
}

.phonetic {
  font-size: 12px;
  font-style: italic;
  color: var(--text-secondary);
}

.pos-label {
  font-size: 11px;
  color: var(--text-secondary);
}

.stars {
  font-size: 10px;
  color: var(--accent-primary);
  letter-spacing: 1px;
}

.tenses-toggle {
  margin-left: auto;
  padding: 3px 8px;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 10px;
  color: var(--text-secondary);
  font-size: 10px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.tenses-toggle:hover,
.tenses-toggle.expanded {
  background: var(--accent-primary);
  color: white;
  border-color: var(--accent-primary);
}

/* Tenses section */
.tenses-section {
  padding: 0 16px 10px 42px;
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
}

.tense-item {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  padding: 4px 8px;
  background: var(--bg-secondary);
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.tense-item:hover {
  background: var(--accent-primary);
}

.tense-item:hover .tense-form,
.tense-item:hover .tense-desc {
  color: white;
}

.tense-form {
  font-size: 12px;
  font-weight: 500;
  color: var(--text-primary);
}

.tense-desc {
  font-size: 10px;
  color: var(--text-secondary);
}

/* Transition animations */
.slide-fade-enter-active {
  transition: all 0.2s ease-out;
}

.slide-fade-leave-active {
  transition: all 0.15s ease-in;
}

.slide-fade-enter-from,
.slide-fade-leave-to {
  transform: translateY(-8px);
  opacity: 0;
}

/* Custom scrollbar */
.results-list::-webkit-scrollbar {
  width: 4px;
}

.results-list::-webkit-scrollbar-track {
  background: transparent;
}

.results-list::-webkit-scrollbar-thumb {
  background: var(--scrollbar-thumb);
  border-radius: 2px;
}

.results-list::-webkit-scrollbar-thumb:hover {
  background: var(--scrollbar-thumb-hover);
}
</style>
