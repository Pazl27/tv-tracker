<template>
  <router-view />
  <ToastContainer />
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import ToastContainer from './components/ToastContainer.vue'
import { useWatchlistStore } from './stores/watchlistStore'

const { initializeWatchlist } = useWatchlistStore()

onMounted(() => {
  // Initialize watchlist data when app loads
  initializeWatchlist()
})
</script>

<style scoped>
</style>
