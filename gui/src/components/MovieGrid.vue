<template>
  <div class="movie-grid">
    <div class="movie-card" v-for="movie in movies" :key="movie.id">
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

// Fetch trending movies
const loadMovies = async () => {
  try {
    movies.value = await fetchMovies(invoke);
  } catch (error) {
    console.error('Failed to load movies:', error);
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

.movie-card {
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
</style>

