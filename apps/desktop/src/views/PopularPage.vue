<template>
  <div>
    <TabBar :activeTab="activeTab" :activeSubTab="activeSubTab" @tab-switched="switchTab" @sub-tab-switched="switchSubTab" @search-input="handleSearchInput" />
    <MovieGrid v-if="activeSubTab === 'movies'" :key="movieGridKey" :searchedMovies="movies" />
    <TvShowGrid v-if="activeSubTab === 'tvShows'" :searchedTvShows="tvShows" />
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted } from 'vue';
import { invoke } from "@tauri-apps/api/core";
import { useRoute, useRouter } from 'vue-router';
import MovieGrid from '../components/popular/MovieGrid.vue';
import TvShowGrid from '../components/popular/TvShowGrid.vue';
import TabBar from '../components/TabBar.vue';
import { searchMovies, searchShows } from '../services/tmdbService';

const route = useRoute();
const router = useRouter();
const activeTab = ref('popular');
const activeSubTab = ref('movies');
const movieGridKey = ref(0);
const searchQuery = ref('');
const movies = ref([]);
const tvShows = ref([]);

const switchTab = (tab: string) => {
  activeTab.value = tab;
};

const switchSubTab = (subTab: string) => {
  activeSubTab.value = subTab;

  // Update URL query parameter to preserve tab state
  router.replace({
    path: '/popular',
    query: { tab: subTab }
  });

  // Load TV shows when switching to TV tab if not already loaded
  if (subTab === 'tvShows' && tvShows.value.length === 0 && !searchQuery.value) {
    loadTvShows();
  }
};

const loadTvShows = async () => {
  try {
    const { fetchTvShows } = await import('../services/tmdbService');
    tvShows.value = await fetchTvShows(invoke);
  } catch (error) {
    console.error('Failed to load TV shows:', error);
  }
};

const handleSearchInput = (query: string) => {
  searchQuery.value = query;

  if (activeSubTab.value === 'movies') {
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
  } catch (error) {
    console.error('Failed to search TV shows:', error);
  }
};

// Watch for route query changes to set the correct tab
watch(() => route.query.tab, (newTab) => {
  if (newTab === 'tvShows') {
    activeSubTab.value = 'tvShows';
  } else if (newTab === 'movies') {
    activeSubTab.value = 'movies';
  }
}, { immediate: true });

onMounted(() => {
  // Check query parameter for initial tab
  if (route.query.tab === 'tvShows') {
    activeSubTab.value = 'tvShows';
    // Load TV shows if coming from navigation with tvShows tab
    if (tvShows.value.length === 0) {
      loadTvShows();
    }
  }
});
</script>
