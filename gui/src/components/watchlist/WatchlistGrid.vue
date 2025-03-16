<template>
  <div class="movie-grid">
    <!-- Skeleton loaders with the same structure as movie cards -->
    <div v-if="loading" class="skeleton-loader" v-for="n in 20" :key="n">
      <div class="skeleton-poster"></div>
      <div class="skeleton-title"></div>
    </div>

    <!-- Watchlist Cards -->
    <div v-else class="movie-card" v-for="item in watchlistItems" :key="item.id">
      <img :src="item.poster_url" :alt="item.title || item.name" class="movie-poster" />
      <h3 class="movie-title">{{ item.title || item.name }}</h3>
      <button class="remove-button" @click="removeFromWatchlist(item)"><i class="minus-icon">-</i></button>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref, onMounted, watch } from 'vue';
import { defineProps } from 'vue';
import { invoke } from "@tauri-apps/api/core";

const props = defineProps<{ watchlistItems: any[] }>();

const loading = ref(true);

const removeFromWatchlist = async (item: any) => {
  try {
    await invoke('remove_from_watchlist', { item });
    console.log('Removed from watchlist:', item);
    // Optionally, you can emit an event to notify the parent component to update the watchlist
  } catch (error) {
    console.error('Failed to remove item from watchlist:', error);
  }
};

onMounted(() => {
  if (props.watchlistItems.length === 0) {
    loading.value = false;
  } else {
    loading.value = false;
  }
});

watch(() => props.watchlistItems, (newItems) => {
  if (newItems && newItems.length > 0) {
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
  position: relative; /* Ensure the button is positioned correctly */
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
</style>
