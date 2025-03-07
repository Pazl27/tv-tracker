<template>
  <div class="movie-grid">
    <div v-if="loading" class="skeleton-loader" v-for="n in 20" :key="n"></div>
    <div v-else class="movie-card" v-for="movie in movies" :key="movie.id">
      <img :src="movie.poster_url" :alt="movie.title" class="movie-poster" />
      <h3 class="movie-title">{{ movie.title }}</h3>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref, onMounted } from 'vue';
import { invoke } from "@tauri-apps/api/core";
import { fetchMovies } from '../services/tmdbService';

const movies = ref<any[]>([]);
const loading = ref(true);

// Fetch trending movies
const loadMovies = async () => {
  try {
    movies.value = await fetchMovies(invoke);
  } catch (error) {
    console.error('Failed to load movies:', error);
  } finally {
    loading.value = false;
  }
};

// Fetch initial movies when component mounts
onMounted(loadMovies);
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
