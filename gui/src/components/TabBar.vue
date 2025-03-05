<!-- src/components/TabBar.vue -->
<template>
  <div class="tabbar">
    <!-- Movies tab -->
    <button
      :class="{ active: activeTab === 'movies' }"
      @click="switchTab('movies')"
    >
      Movies
    </button>
    
    <!-- TV Shows tab -->
    <button
      :class="{ active: activeTab === 'tvShows' }"
      @click="switchTab('tvShows')"
    >
      TV Shows
    </button>
  </div>
</template>

<script lang="ts" setup>
import { defineProps, defineEmits } from 'vue';

const emit = defineEmits<{
  (e: 'tab-switched', tab: string): void;
}>();

const props = defineProps({
  activeTab: String,
});

const switchTab = (tab: string) => {
  emit('tab-switched', tab);
};
</script>

<style scoped>
.tabbar {
  display: flex;
  justify-content: center;
  margin-bottom: 16px;
}

.tabbar button {
  padding: 8px 16px;
  margin: 0 8px;
  cursor: pointer;
  border: none;
  background-color: #f0f0f0;
  border-radius: 4px;
  font-size: 16px;
}

/* Highlight active tab */
.tabbar button.active {
  background-color: #007bff;
  color: white;
}
</style>
