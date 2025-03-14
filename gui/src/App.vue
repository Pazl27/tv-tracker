<template>
  <div>
    <div v-if="showApiKeyInput" class="overlay">
      <div class="api-key-input-container">
        <input type="text" v-model="apiKey" placeholder="Invalid API Key, please enter a valid one"
          @keyup.enter="saveApiKey" class="api-key-input" />
      </div>
    </div>
    <div :class="{ blurred: showApiKeyInput }">
      <TabBar :activeTab="activeTab" @tab-switched="switchTab" @search-input="handleSearchInput" />
      <MovieGrid v-if="activeTab === 'movies'" :key="movieGridKey" :searchedMovies="movies" />
      <TvShowGrid v-if="activeTab === 'tvShows'" :searchedTvShows="tvShows" />
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref, onMounted } from 'vue';
import { invoke } from "@tauri-apps/api/core";
import MovieGrid from './components/MovieGrid.vue';
import TvShowGrid from './components/TvShowGrid.vue';
import TabBar from './components/TabBar.vue';
import { searchMovies, searchShows } from './services/tmdbService';

const activeTab = ref('movies');
const showApiKeyInput = ref(false);
const apiKey = ref('');
const movieGridKey = ref(0);
const searchQuery = ref('');
const movies = ref([]);
const tvShows = ref([]);

const switchTab = (tab: string) => {
  activeTab.value = tab;
};

const saveApiKey = async () => {
  try {
    await invoke('add_api_key', { key: apiKey.value });
  } catch (error) {
    alert('Something went wrong saving the API key');
  }

  const isValid = await invoke('valid_key', { apiKey: apiKey.value }).catch(() => false);
  if (isValid) {
    showApiKeyInput.value = false;
    movieGridKey.value += 1;
  } else {
    alert('Invalid API Key, please try again.');
  }
};

const handleSearchInput = (query: string) => {
  searchQuery.value = query;

  if (activeTab.value === 'movies') {
    searchMoviesHandler(query);
  } else {
    searchTvShowsHandler(query);
  }
};

const searchMoviesHandler = async (query: string) => {
  try {
    movies.value = await searchMovies(invoke, query);
  } catch (error) {
    console.error('Failed to search movies:', error);
  }
};

const searchTvShowsHandler = async (query: string) => {
  try {
    tvShows.value = await searchShows(invoke, query);
    console.log(tvShows.value);
  } catch (error) {
    console.error('Failed to search TV shows:', error);
  }
}

onMounted(async () => {
  const isValid = await invoke('valid_key').catch(() => false);
  if (!isValid) {
    showApiKeyInput.value = true;
  }
});
</script>

<style scoped>
.overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
}

.api-key-input-container {
  background: var(--background-color);
  padding: 40px;
  border-radius: 12px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.2);
  width: 600px;
  text-align: center;
}

.api-key-input {
  width: 100%;
  padding: 15px;
  font-size: 18px;
  border: 1px solid #ccc;
  border-radius: 8px;
  box-sizing: border-box;
}

.blurred {
  filter: blur(5px);
}
</style>
