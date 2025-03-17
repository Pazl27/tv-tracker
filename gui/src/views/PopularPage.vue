<template>
  <div>
    <TabBar :activeTab="activeTab" :activeSubTab="activeSubTab" @tab-switched="switchTab" @sub-tab-switched="switchSubTab" @search-input="handleSearchInput" />
    <MovieGrid v-if="activeSubTab === 'movies'" :key="movieGridKey" :searchedMovies="movies" />
    <TvShowGrid v-if="activeSubTab === 'tvShows'" :searchedTvShows="tvShows" />
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from "@tauri-apps/api/core";
import MovieGrid from '../components/popular/MovieGrid.vue';
import TvShowGrid from '../components/popular/TvShowGrid.vue';
import TabBar from '../components/TabBar.vue';
import { searchMovies, searchShows } from '../services/tmdbService';

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
    console.log(tvShows.value);
  } catch (error) {
    console.error('Failed to search TV shows:', error);
  }
};
</script>
