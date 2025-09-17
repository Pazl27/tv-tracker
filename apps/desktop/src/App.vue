<template>
  <router-view />
  <ToastContainer />
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import ToastContainer from './components/ToastContainer.vue'
import { useWatchlistStore } from './stores/watchlistStore'
import { useRatingStore } from './stores/ratingStore'

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
