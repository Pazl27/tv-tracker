
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

const movie = ref<any>(null);

onMounted(() => {
  movie.value = JSON.parse(localStorage.getItem('selectedMovie') || 'null');
  if (!movie.value) {
    console.error('Movie data is missing. Ensure it is passed via router state.');
  }
});
</script>

<style scoped>
.movie-poster {
  width: 100%;
  height: 300px;
  object-fit: cover;
}
</style>
