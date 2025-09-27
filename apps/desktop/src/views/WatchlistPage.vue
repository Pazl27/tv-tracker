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
import TabBar from '../components/TabBar.vue';
import WatchlistMovieGrid from '../components/watchlist/WatchlistMovieGrid.vue';
import WatchlistTvShowGrid from '../components/watchlist/WatchlistTvShowGrid.vue';
import { useWatchlistStore } from '../stores/watchlistStore';
import { ref, onMounted, watch } from 'vue';
import { useRoute, useRouter } from 'vue-router';

const route = useRoute();
const router = useRouter();
const activeSubTab = ref('movies');
const searchQuery = ref('');

// Use the watchlist store
const {
  watchlistTvShows,
  isLoadingMovies,
  isLoadingTvShows,
  loadWatchlistMovies,
  loadWatchlistTvShows
} = useWatchlistStore();

const handleSubTabSwitched = (subTab: string) => {
  activeSubTab.value = subTab;

  // Update URL query parameter to preserve tab state
  router.replace({
    path: '/watchlist',
    query: { tab: subTab }
  });

  // Load data when switching to TV shows if not loaded yet
  if (subTab === 'tvShows' && watchlistTvShows.value.length === 0 && !isLoadingTvShows.value) {
    loadWatchlistTvShows();
  }
};

const handleSearchInput = (query: string) => {
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

const searchMoviesHandler = async (_query: string) => {
  try {
    // This searches all movies, not just watchlist
    // The component will filter the watchlist based on searchQuery
  } catch (error) {
    console.error('Failed to search movies:', error);
  }
};

const searchTvShowsHandler = async (_query: string) => {
  try {
    // This searches all TV shows, not just watchlist
    // The component will filter the watchlist based on searchQuery
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
  }

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
