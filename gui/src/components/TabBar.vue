<template>
  <div class="tabbar-container">
    <!-- Search Bar -->
    <div class="search-bar">
      <input
        v-model="searchQuery"
        type="text"
        placeholder="Search..."
        @keydown.enter="onSearchInput"
      />
    </div>

    <!-- Tabs -->
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
  </div>
</template>

<script lang="ts" setup>
import { defineProps, defineEmits, ref } from 'vue';

const emit = defineEmits<{
  (e: 'tab-switched', tab: string): void;
  (e: 'search-input', query: string): void;
}>();

const props = defineProps({
  activeTab: String,
});

const searchQuery = ref(''); 

const switchTab = (tab: string) => {
  emit('tab-switched', tab);
};

const onSearchInput = () => {
  emit('search-input', searchQuery.value);
};
</script>

<style scoped>
.tabbar-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  margin-bottom: 16px;
}

.search-bar {
  margin-bottom: 16px;
  width: 100%;
  max-width: 800px;
}

.search-bar input {
  width: 100%;
  padding: 12px 16px;
  font-size: 16px;
  border: 2px solid #ccc;
  border-radius: 24px;
  box-sizing: border-box;
  outline: none;
  transition: all 0.3s ease;
}

.search-bar input:focus {
  border-color: #007bff;
  box-shadow: 0 0 5px rgba(0, 123, 255, 0.5);
}

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
