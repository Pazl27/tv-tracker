<template>
  <div>
    <TabBar :activeTab="activeTab" @tab-switched="switchTab" />
    <MovieGrid v-if="activeTab === 'movies'" />
    <TvShowGrid v-if="activeTab === 'tvShows'" />
  </div>
</template>

<script lang="ts" setup>
import { ref } from 'vue';
import MovieGrid from './components/MovieGrid.vue';
import TvShowGrid from './components/TvShowGrid.vue';
import TabBar from './components/TabBar.vue';

const activeTab = ref('movies');

// Function to switch between tabs
const switchTab = (tab: string) => {
  activeTab.value = tab;
};
</script>

<style>
</style>

