<template>
  <div class="tabbar-container">
    <!-- Home Link -->
    <div class="home-link" @click="goHome">
      Home
    </div>

    <!-- Search Bar -->
    <div class="search-bar">
      <input
        v-model="searchQuery"
        type="text"
        placeholder="Search..."
        @keydown.enter="onSearchInput"
      />
    </div>

    <!-- Sub Tabs -->
    <div class="sub-tabbar">
      <!-- Movies sub-tab -->
      <button
        :class="{ active: activeSubTab === 'movies' }"
        @click="switchSubTab('movies')"
      >
        Movies
      </button>

      <!-- TV Shows sub-tab -->
      <button
        :class="{ active: activeSubTab === 'tvShows' }"
        @click="switchSubTab('tvShows')"
      >
        TV Shows
      </button>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { defineProps, defineEmits, ref } from 'vue';
import { useRouter } from 'vue-router';

const emit = defineEmits<{
  (e: 'sub-tab-switched', subTab: string): void;
  (e: 'search-input', query: string): void;
}>();

const props = defineProps({
  activeSubTab: String,
});

const searchQuery = ref('');
const router = useRouter();

const switchSubTab = (subTab: string) => {
  emit('sub-tab-switched', subTab);
};

const onSearchInput = () => {
  emit('search-input', searchQuery.value);
};

const goHome = () => {
  router.push('/');
};
</script>

<style scoped>
.tabbar-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  margin-bottom: 16px;
}

.home-link {
  margin-bottom: 16px;
  cursor: pointer;
  font-size: 24px;
  font-weight: bold;
  color: #007bff;
}

.home-link:hover {
  text-decoration: underline;
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

.sub-tabbar {
  display: flex;
  justify-content: center;
  margin-bottom: 16px;
}

.sub-tabbar button {
  padding: 8px 16px;
  margin: 0 8px;
  cursor: pointer;
  border: none;
  background-color: #f0f0f0;
  border-radius: 4px;
  font-size: 16px;
}

/* Highlight active tab */
.sub-tabbar button.active {
  background-color: #007bff;
  color: white;
}
</style>
