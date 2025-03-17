<template>
  <div class="movie-grid">
    <!-- Skeleton loaders with the same structure as TV show cards -->
    <div v-if="loading" class="skeleton-loader" v-for="n in 20" :key="n">
      <div class="skeleton-poster"></div>
      <div class="skeleton-title"></div>
    </div>

    <!-- TV Show Cards -->
    <div v-else class="movie-card" v-for="show in watchlistTvShows" :key="show.id">
      <img :src="show.poster_url" :alt="show.name" class="movie-poster" />
      <h3 class="movie-title">{{ show.name }}</h3>
      <button class="remove-button" @click="removeFromWatchlist(show)"><i class="minus-icon">-</i></button>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref, onMounted, watch } from 'vue';
import { invoke } from "@tauri-apps/api/core";
import { defineProps } from 'vue';

const props = defineProps<{ watchlistTvShows: any[] }>();

const watchlistTvShows = ref<any[]>(props.watchlistTvShows || []);
const loading = ref(true);

const removeFromWatchlist = async (show: any) => {
  try {
    await invoke('remove_tv_show_from_watchlist', { show });
    console.log('Removed from watchlist:', show);
  } catch (error) {
    console.error('Failed to remove TV show from watchlist:', error);
  }
};

onMounted(() => {
  if (watchlistTvShows.value.length === 0) {
    loading.value = false;
  } else {
    loading.value = false;
  }
});

watch(() => props.watchlistTvShows, (newTvShows) => {
  if (newTvShows && newTvShows.length > 0) {
    watchlistTvShows.value = newTvShows;
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
  border-radius: 8px;
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

.remove-button {
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

.movie-card:hover .remove-button {
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
