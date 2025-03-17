<template>
  <div class="movie-grid">
    <!-- Skeleton loaders with the same structure as TV show cards -->
    <div v-if="loading" class="skeleton-loader" v-for="n in 60" :key="n">
      <div class="skeleton-poster"></div>
      <div class="skeleton-title"></div>
    </div>

    <!-- TV Show Cards -->
    <div v-else class="movie-card" v-for="show in tvShows" :key="show.id">
      <img :src="show.poster_url" :alt="show.name" class="movie-poster" />
      <h3 class="movie-title">{{ show.name }}</h3>
      <button class="add-button" @click="addToWatchlist(show)"><i class="plus-icon">+</i></button>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref, onMounted, watch } from 'vue';
import { invoke } from "@tauri-apps/api/core";
import { fetchTvShows } from '../../services/tmdbService';
import { defineProps } from 'vue';

const props = defineProps<{ searchedTvShows: any[] }>();

const tvShows = ref<any[]>(props.searchedTvShows || []);
const loading = ref(true);

// Fetch trending TV shows
const loadTvShows = async () => {
  try {
    tvShows.value = await fetchTvShows(invoke);
    console.log(tvShows.value.length);
  } catch (error) {
    console.error('Failed to load TV shows:', error);
  } finally {
    loading.value = false;
  }
};

const addToWatchlist = async (show: any) => {
  try {
    await invoke('add_show_to_watchlist', { show });
    console.log('Added to watchlist:', show);
  } catch (error) {
    console.error('Failed to add show to watchlist:', error);
  }
};

// Fetch initial TV shows when component mounts
onMounted(() => {
  if (tvShows.value.length === 0) {
    loadTvShows();
  } else {
    loading.value = false;
  }
});

watch(() => props.searchedTvShows, (newTvShows) => {
  if (newTvShows && newTvShows.length > 0) {
    tvShows.value = newTvShows;
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
  border-radius: 3%;
  background: var(--color-background-dark);
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
