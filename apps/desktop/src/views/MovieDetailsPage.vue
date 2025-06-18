<template>
  <div class="page" v-if="movie">
    <div class="container">
      <div class="movie-details">
        <img :src="movie.poster_url" :alt="movie.title" class="movie-poster" />
        <div class="movie-info">
          <h1 class="movie-title">{{ movie.title }}</h1>
          <p class="movie-runtime">{{ movie.runtime }} minutes</p>
          <p class="movie-release-date">Release Date: {{ movie.release_date }}</p>
          <p class="movie-genres">Genres: {{ movie.genres.join(', ') }}</p>
          <p class="movie-overview">{{ movie.overview }}</p>
          <div class="movie-rating">
            <span class="star">★</span>
            <span>{{ movie.vote_average }} / 10</span>
          </div>
        </div>
      </div>
    </div>
  </div>
  <div v-else>
    <p class="no-data">Movie data is not available.</p>
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

<style scoped>
.container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 20px;
}

.movie-details {
  display: flex;
  flex-direction: column;
  align-items: center;
}

.movie-poster {
  width: 100%;
  max-width: 300px;
  border-radius: 10px;
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
}

.movie-info {
  margin-top: 20px;
  text-align: center;
}

.movie-title {
  font-size: 2rem;
  font-weight: bold;
  margin-bottom: 10px;
}

.movie-runtime,
.movie-release-date,
.movie-genres,
.movie-overview {
  color: #555;
  margin-bottom: 10px;
}

.movie-rating {
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 1.2rem;
  color: #333;
}

.star {
  color: #f39c12;
  margin-right: 5px;
}

.no-data {
  text-align: center;
  color: #999;
  font-size: 1.2rem;
  margin-top: 50px;
}
</style>
