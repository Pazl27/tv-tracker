<template>
  <TabBar
    :activeSubTab="activeSubTab"
    @sub-tab-switched="handleSubTabSwitched"
    @search-input="handleSearchInput"
  />
  <WatchlistMovieGrid
    :watchlistMovies="watchlistMovies"
    :searchQuery="searchQuery"
  />
</template>

<script lang="ts" setup>
import { ref } from 'vue';
import { invoke } from "@tauri-apps/api/core";
import TabBar from '../components/TabBar.vue';
import WatchlistMovieGrid from '../components/watchlist/WatchlistMovieGrid.vue';
import { searchMovies, searchShows } from '../services/tmdbService';

const activeSubTab = ref('movies');
const searchQuery = ref('');
const watchlistMovies = ref<any[]>([]);

const handleSubTabSwitched = (subTab: string) => {
  activeSubTab.value = subTab;
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
    watchlistMovies.value = await searchMovies(invoke, query);
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
};

</script>
