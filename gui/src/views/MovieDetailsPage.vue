<template>
  <div class="page" v-if="movie">
    <img :src="movie.poster_url" :alt="movie.title" class="movie-poster" />
    <h1>{{ movie.title }}</h1>
    <p>{{ movie.description }}</p>
  </div>
  <div v-else>
    <p>Movie data is not available.</p>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useRoute } from 'vue-router';
import { getMovieDetails } from '../services/tmdbService';
import { invoke } from '@tauri-apps/api/core';

const movie = ref<any>(null);
const route = useRoute();

const fetchMovie = async (movieId: number) => {
  try {
    movie.value = await getMovieDetails(invoke, movieId);
  } catch (error) {
    console.error('Failed to load movie details:', error);
  }
}

onMounted(() => {
  const movieId = route.params.id;
  if (movieId) {
    fetchMovie(Number(movieId));
  } else {
    console.error('Movie ID is missing. Ensure it is passed via router state.');
  }
});
</script>
