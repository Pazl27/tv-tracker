<template>
  <div class="movie-grid">
    <div v-if="loading" class="skeleton-loader" v-for="n in 20" :key="n"></div>
    <div v-else class="movie-card" v-for="show in tvShows" :key="show.id">
      <img :src="show.poster_url" :alt="show.name" class="movie-poster" />
      <h3 class="movie-title">{{ show.name }}</h3>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref, onMounted } from 'vue';
import { invoke } from "@tauri-apps/api/core";
import { fetchTvShows } from '../services/tmdbService';

const tvShows = ref<any[]>([]);
const loading = ref(true);

// Fetch trending TV shows
const loadTvShows = async () => {
  try {
    tvShows.value = await fetchTvShows(invoke);
  } catch (error) {
    console.error('Failed to load TV shows:', error);
  } finally {
    loading.value = false;
  }
};

// Fetch initial TV shows when component mounts
onMounted(loadTvShows);
</script>

<style scoped>
.movie-grid {
  display: flex;
  flex-wrap: wrap;
  gap: 16px;
}

.movie-card, .skeleton-loader {
  flex: 1 1 calc(25% - 16px);
  box-sizing: border-box;
  border: 1px solid #ddd;
  border-radius: 8px;
  overflow: hidden;
  text-align: center;
}

.movie-poster {
  width: 100%;
  height: auto;
}

.movie-title {
  margin: 8px 0;
  font-size: 16px;
}

.skeleton-loader {
  background: #e0e0e0;
  height: 300px; /* Adjust height as needed */
  animation: pulse 1.5s infinite;
}

@keyframes pulse {
  0% {
    background-color: #e0e0e0;
  }
  50% {
    background-color: #f0f0f0;
  }
  100% {
    background-color: #e0e0e0;
  }
}
</style>
