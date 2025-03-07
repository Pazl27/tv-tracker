<template>
  <div class="movie-grid">
    <!-- Skeleton loaders with the same structure as movie cards -->
    <div v-if="loading" class="skeleton-loader" v-for="n in 20" :key="n">
      <div class="skeleton-poster"></div>
      <div class="skeleton-title"></div>
    </div>
    
    <!-- Movie Cards -->
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
  height: 300px;
  object-fit: cover;
}

.movie-title {
  margin: 8px 0;
  font-size: 16px;
}

/* Skeleton Loader Styles */
.skeleton-loader {
  background: #e0e0e0;
}

.skeleton-poster {
  width: 100%;
  height: 300px; /* Match the height of the image */
  background: #f0f0f0;
}

.skeleton-title {
  width: 60%;
  height: 20px;
  margin: 10px auto;
  background: #f0f0f0;
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
