<template>
  <div class="movie-grid">
    <!-- Skeleton loaders with the same structure as TV show cards -->
    <div v-if="loading" class="skeleton-loader" v-for="n in 20" :key="n">
      <div class="skeleton-poster"></div>
      <div class="skeleton-title"></div>
    </div>
    
    <!-- TV Show Cards -->
    <div v-else class="movie-card" v-for="show in tvShows" :key="show.id">
      <img :src="show.poster_url" :alt="show.name" class="movie-poster" />
      <h3 class="movie-title">{{ show.name }}</h3>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref, onMounted, watch } from 'vue';
import { invoke } from "@tauri-apps/api/core";
import { fetchTvShows } from '../services/tmdbService';
import { defineProps } from 'vue';

const props = defineProps<{ searchedTvShows: any[] }>();

const tvShows = ref<any[]>(props.searchedTvShows || []);
const loading = ref(true);

// Fetch trending TV shows
const loadTvShows = async () => {
  try {
    tvShows.value = await fetchTvShows(invoke);
  } catch (error) {
    console.error('Failed to load TV shows:', error);
  } finally {
    loading.value = false;
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
  display: flex;
  flex-wrap: wrap;
  gap: 16px;
}

.movie-card, .skeleton-loader {
  flex: 1 1 calc(25% - 16px);
  box-sizing: border-box;
  border: 2px solid var(--color-border);
  border-radius: 3%;
  background: var(--color-background-dark);
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
  background: var (--color-background-dark);
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
