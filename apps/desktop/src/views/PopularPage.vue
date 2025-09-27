<template>
  <div>
    <TabBar :active-tab="activeTab" :active-sub-tab="activeSubTab" @tab-switched="switchTab" @sub-tab-switched="switchSubTab" @search-input="handleSearchInput" />
    <MovieGrid v-if="activeSubTab === 'movies'" :key="movieGridKey" :searched-movies="movies" />
    <TvShowGrid v-if="activeSubTab === 'tvShows'" :searched-tv-shows="tvShows" />
  </div>
</template>

<script setup lang="ts">
import TabBar from '../components/TabBar.vue';
import MovieGrid from '../components/popular/MovieGrid.vue';
import TvShowGrid from '../components/popular/TvShowGrid.vue';
import { searchMovies, searchShows } from '../services/tmdbService';
import { invoke } from "@tauri-apps/api/core";
import { ref, watch, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';

const route = useRoute();
const router = useRouter();
const activeTab = ref('popular');
const activeSubTab = ref('movies');
const movieGridKey = ref(0);
const searchQuery = ref('');
const movies = ref<any[]>([]);
const tvShows = ref<any[]>([]);

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
    movies.value = await searchMovies(invoke, query) as any[];
  } catch (error) {
    console.error('Failed to search movies:', error);
  }
};

const searchTvShowsHandler = async (query: string) => {
  try {
    tvShows.value = await searchShows(invoke, query) as any[];
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
