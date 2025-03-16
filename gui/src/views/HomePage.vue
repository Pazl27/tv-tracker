<template>
  <div class="home-page">
    <h1>Welcome to TV Tracker</h1>
    <div class="navigation">
      <router-link to="/popular">Popular</router-link>
      <router-link to="/watchlist">Watchlist</router-link>
    </div>
  </div>
</template>

<script setup lang="ts">
</script>

<style scoped>
.home-page {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100vh;
}

.navigation {
  display: flex;
  gap: 20px;
  margin-top: 20px;
}

.navigation a {
  padding: 10px 20px;
  background-color: #007bff;
  color: white;
  text-decoration: none;
  border-radius: 5px;
}

.navigation a:hover {
  background-color: #0056b3;
}
</style>
