<template>
  <div>
    <TabBar :activeTab="activeTab" :activeSubTab="activeSubTab" @tab-switched="switchTab" @sub-tab-switched="switchSubTab" @search-input="handleSearchInput" />
    <WatchlistGrid v-if="activeSubTab === 'movies'" :watchlistItems="watchlistMovies" />
    <WatchlistGrid v-if="activeSubTab === 'tvShows'" :watchlistItems="watchlistTvShows" />
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from "@tauri-apps/api/core";
import WatchlistGrid from '../components/watchlist/WatchlistGrid.vue';
import TabBar from '../components/TabBar.vue';

const activeTab = ref('watchlist');
const activeSubTab = ref('movies');
const watchlistMovies = ref([]);
const watchlistTvShows = ref([]);

const switchTab = (tab: string) => {
  activeTab.value = tab;
};

const switchSubTab = (subTab: string) => {
  activeSubTab.value = subTab;
};

const handleSearchInput = (query: string) => {
  // Implement search logic for watchlist
};
</script>
