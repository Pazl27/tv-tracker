<template>
  <router-view />
  <ToastContainer />
</template>

<script setup lang="ts">
import ToastContainer from './components/ToastContainer.vue'
import { useRatingStore } from './stores/ratingStore'
import { useWatchlistStore } from './stores/watchlistStore'
import { onMounted } from 'vue'

const { initializeWatchlist } = useWatchlistStore()
const { initializeRatings } = useRatingStore()

onMounted(() => {
  // Initialize watchlist and ratings data when app loads
  initializeWatchlist()
  initializeRatings()
})
</script>

<style scoped>
</style>
