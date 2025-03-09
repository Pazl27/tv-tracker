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
import { ref, onMounted, watch } from 'vue';
import { invoke } from "@tauri-apps/api/core";
import { fetchMovies } from '../services/tmdbService';
import { defineProps } from 'vue';

const props = defineProps<{ searchedMovies: any[] }>();

const movies = ref<any[]>(props.searchedMovies || []);
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

onMounted(() => {
  if (movies.value.length === 0) {
    loadMovies();
  } else {
    loading.value = false;
  }
});

watch(() => props.searchedMovies, (newMovies) => {
  if (newMovies && newMovies.length > 0) {
    movies.value = newMovies;
    loading.value = false;
  }
});
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
  border: 2px solid var(--color-border);
  background: var(--color-background-dark);
  border-radius: 8px;
  overflow: hidden;
  text-align: center;
}

.movie-card:hover {
  border: 2px solid var(--color-border-hover);
}

.skeleton-loader:hover {
  border: 2px solid var(--color-border-hover);
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
  background: var(--color-background-light);
}

.skeleton-poster {
  width: 100%;
  height: 300px; 
  background: var(--color-background-dark);
}

.skeleton-title {
  width: 60%;
  height: 20px;
  margin: 10px auto;
  background: var(--color-background-dark);
}

@keyframes pulse {
  0% {
    background-color: var(--color-background-light);
  }
  50% {
    background-color: var(--color-background-dark);
  }
  100% {
    background-color: var(--color-background-light);
  }
}
</style>
