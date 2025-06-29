<template>
  <div class="movie-grid">
    <!-- Skeleton loaders with the same structure as movie cards -->
    <div v-if="loading" class="skeleton-loader" v-for="n in 20" :key="n">
      <div class="skeleton-poster"></div>
      <div class="skeleton-title"></div>
    </div>

    <!-- Movie Cards -->
    <div v-else class="movie-card" v-for="movie in movies" :key="movie.id" @click="goToMovieDetails(movie)">
      <img :src="movie.poster_url" :alt="movie.title" class="movie-poster" />
      <h3 class="movie-title">{{ movie.title }}</h3>
      <button class="add-button" @click.stop="addToWatchlist(movie)"><i class="plus-icon">+</i></button>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref, onMounted, watch } from 'vue';
import { useRouter } from 'vue-router';
import { invoke } from "@tauri-apps/api/core";
import { fetchMovies } from '../../services/tmdbService';
import { defineProps } from 'vue';

const props = defineProps<{ searchedMovies: any[] }>();

const movies = ref<any[]>(props.searchedMovies || []);
const loading = ref(true);

const router = useRouter();

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

const addToWatchlist = async (movie: any) => {
  try {
    await invoke('add_movie_to_watchlist', { movie });
    console.log('Added to watchlist:', movie);
  } catch (error) {
    console.error('Failed to add movie to watchlist:', error);
  }
};

const goToMovieDetails = (movie: any) => {
  localStorage.setItem('selectedMovie', JSON.stringify(movie));
  router.push({
    name: 'MovieDetails',
    params: { id: movie.id }
  });
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
  display: grid;
  gap: 16px;
  grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
}

.movie-card, .skeleton-loader {
  box-sizing: border-box;
  border: 2px solid var(--color-border);
  background: var(--color-background-dark);
  border-radius: 8px;
  overflow: hidden;
  text-align: center;
  position: relative;
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

.add-button {
  display: none;
  position: absolute;
  bottom: 10px;
  right: 10px;
  background-color: var(--color-border-hover);
  border: none;
  border-radius: 50%;
  width: 40px;
  height: 40px;
  color: white;
  font-size: 24px;
  cursor: pointer;
}

.movie-card:hover .add-button {
  display: block;
}

/* Responsive Styles */
@media (min-width: 1200px) {
  .movie-grid {
    grid-template-columns: repeat(8, 1fr); /* 8 cards per row */
  }
}

@media (min-width: 992px) and (max-width: 1199px) {
  .movie-grid {
    grid-template-columns: repeat(6, 1fr); /* 6 cards per row */
  }
}

@media (min-width: 768px) and (max-width: 991px) {
  .movie-grid {
    grid-template-columns: repeat(4, 1fr); /* 4 cards per row */
  }
}

@media (max-width: 767px) {
  .movie-grid {
    grid-template-columns: repeat(2, 1fr); /* 2 cards per row */
  }
}

@media (max-width: 480px) {
  .movie-grid {
    grid-template-columns: repeat(1, 1fr); /* 1 card per row */
  }
}
</style>
