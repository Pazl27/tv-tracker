<template>
  <div>
    <!-- Tab Bar -->
    <div class="tabbar">
      <button
        :class="{ active: activeTab === 'movies' }"
        @click="switchTab('movies')"
      >
        Movies
      </button>
      <button
        :class="{ active: activeTab === 'tvShows' }"
        @click="switchTab('tvShows')"
      >
        TV Shows
      </button>
    </div>

    <!-- Movie or TV Show Grid -->
    <div v-if="activeTab === 'movies'" class="movie-grid">
      <div class="movie-card" v-for="movie in movies" :key="movie.id">
        <img :src="movie.poster_url" :alt="movie.title" class="movie-poster" />
        <h3 class="movie-title">{{ movie.title }}</h3>
      </div>
    </div>

    <div v-if="activeTab === 'tvShows'" class="movie-grid">
      <div class="movie-card" v-for="show in tvShows" :key="show.id">
        <img :src="show.poster_url" :alt="show.title" class="movie-poster" />
        <h3 class="movie-title">{{ show.name }}</h3>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref, onMounted } from 'vue';
import { invoke } from "@tauri-apps/api/core";

interface Movie {
  id: number;
  title: string;
  poster_path: string;
  poster_url: string;
}

interface TvShow {
  id: number;
  name: string;
  poster_path: string;
  poster_url: string;
}

const activeTab = ref('movies');
const movies = ref<Movie[]>([]);
const tvShows = ref<TvShow[]>([]);

// Function to switch tabs
const switchTab = async (tab: string) => {
  activeTab.value = tab;

  if (tab === 'movies') {
    await fetchMovies();
  } else if (tab === 'tvShows') {
    await fetchTvShows();
  }
};

// Fetch trending movies
const fetchMovies = async () => {
  try {
    const result: Movie[] = await invoke('get_trending_movies');
    movies.value = result.map((movie) => ({
      ...movie,
      poster_url: `https://image.tmdb.org/t/p/original${movie.poster_path}`,
    }));
  } catch (error) {
    console.error('Failed to fetch trending movies:', error);
  }
};

// Fetch trending TV shows
const fetchTvShows = async () => {
  try {
    const result: TvShow[] = await invoke('get_trending_tv');
    tvShows.value = result.map((show) => ({
      ...show,
      poster_url: `https://image.tmdb.org/t/p/original${show.poster_path}`,
    }));
  } catch (error) {
    console.error('Failed to fetch trending TV shows:', error);
  }
};

// Fetch initial data when component mounts
onMounted(async () => {
  await fetchMovies(); // Default to fetching movies
});
</script>

<style scoped>
/* Tab Bar */
.tabbar {
  display: flex;
  justify-content: center;
  margin-bottom: 16px;
}

.tabbar button {
  padding: 8px 16px;
  margin: 0 8px;
  cursor: pointer;
  border: none;
  background-color: #f0f0f0;
  border-radius: 4px;
  font-size: 16px;
}

.tabbar button.active {
  background-color: #007bff;
  color: white;
}

/* Movie Grid */
.movie-grid {
  display: flex;
  flex-wrap: wrap;
  gap: 16px;
}

.movie-card {
  flex: 1 1 calc(25% - 16px);
  box-sizing: border-box;
  border: 1px solid #ddd;
  border-radius: 8px;
  overflow: hidden;
  text-align: center;
}

.movie-poster {
  width: 100%;
  height: auto;
}

.movie-title {
  margin: 8px 0;
  font-size: 16px;
}
</style>
