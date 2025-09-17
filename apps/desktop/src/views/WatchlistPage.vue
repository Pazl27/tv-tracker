<template>
  <TabBar
    :activeSubTab="activeSubTab"
    @sub-tab-switched="handleSubTabSwitched"
    @search-input="handleSearchInput"
  />

  <div v-if="activeSubTab === 'movies'">
    <WatchlistMovieGrid
      :searchQuery="searchQuery"
    />
  </div>
  <div v-else>
    <WatchlistTvShowGrid
      :searchQuery="searchQuery"
    />
  </div>
</template>

<script lang="ts" setup>
import { ref, onMounted, computed } from 'vue';
import { invoke } from "@tauri-apps/api/core";
import TabBar from '../components/TabBar.vue';
import WatchlistMovieGrid from '../components/watchlist/WatchlistMovieGrid.vue';
import WatchlistTvShowGrid from '../components/watchlist/WatchlistTvShowGrid.vue';
import { searchMovies, searchShows } from '../services/tmdbService';
import { useWatchlistStore } from '../stores/watchlistStore';

const activeSubTab = ref('movies');
const searchQuery = ref('');
const showDebug = ref(true); // Enable debugging

// Use the watchlist store
const {
  watchlistMovies,
  watchlistTvShows,
  isLoadingMovies,
  isLoadingTvShows,
  loadWatchlistMovies,
  loadWatchlistTvShows
} = useWatchlistStore();

const handleSubTabSwitched = (subTab: string) => {
  console.log('Switching to tab:', subTab);
  activeSubTab.value = subTab;

  // Load data when switching to TV shows if not loaded yet
  if (subTab === 'tvShows' && watchlistTvShows.value.length === 0 && !isLoadingTvShows.value) {
    console.log('Loading TV shows for watchlist tab...');
    loadWatchlistTvShows();
  }
};

const handleSearchInput = (query: string) => {
  console.log('Search input:', query, 'for tab:', activeSubTab.value);
  searchQuery.value = query;

  // Note: Search within watchlist is handled by the individual components
  // The search here would be for global search, but that's not the current behavior
  // keeping the original logic for now but it might need adjustment
  if (activeSubTab.value === 'movies') {
    searchMoviesHandler(query);
  } else {
    searchTvShowsHandler(query);
  }
};

const searchMoviesHandler = async (query: string) => {
  try {
    // This searches all movies, not just watchlist
    // The component will filter the watchlist based on searchQuery
    console.log('Searching movies globally:', query);
  } catch (error) {
    console.error('Failed to search movies:', error);
  }
};

const searchTvShowsHandler = async (query: string) => {
  try {
    // This searches all TV shows, not just watchlist
    // The component will filter the watchlist based on searchQuery
    console.log('Searching TV shows globally:', query);
  } catch (error) {
    console.error('Failed to search TV shows:', error);
  }
};

onMounted(() => {
  console.log('WatchlistPage mounted');
  console.log('Initial movies in watchlist:', watchlistMovies.value.length);
  console.log('Initial TV shows in watchlist:', watchlistTvShows.value.length);

  // Force reload watchlist data to ensure it's fresh
  if (!isLoadingMovies.value) {
    loadWatchlistMovies();
  }
  if (!isLoadingTvShows.value) {
    loadWatchlistTvShows();
  }
});
</script>

<style scoped>
.debug-info {
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-medium);
  padding: var(--spacing-md);
  margin: var(--spacing-md);
  font-family: monospace;
  font-size: 0.875rem;
  color: var(--color-text-secondary);
}

.debug-info p {
  margin: var(--spacing-xs) 0;
}
</style>
