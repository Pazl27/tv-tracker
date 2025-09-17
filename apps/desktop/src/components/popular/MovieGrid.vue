<template>
  <div class="movie-grid-container">
    <!-- Skeleton loaders -->
    <div v-if="loading" class="movie-grid">
      <div v-for="n in 20" :key="n" class="skeleton-card">
        <div class="skeleton-poster animate-shimmer"></div>
        <div class="skeleton-content">
          <div class="skeleton-title animate-shimmer"></div>
          <div class="skeleton-subtitle animate-shimmer"></div>
        </div>
      </div>
    </div>

    <!-- Movie Cards -->
    <div v-else class="movie-grid">
      <div 
        v-for="movie in movies" 
        :key="movie.id" 
        class="movie-card"
        @click="goToMovieDetails(movie)"
      >
        <div class="movie-poster-container">
          <LazyImage
            :src="movie.poster_url"
            :alt="movie.title"
            aspect-ratio="2/3"
            quality="medium"
            :show-spinner="true"
            root-margin="100px"
            class="movie-poster"
            @load="onImageLoad"
            @error="onImageError"
            @visible="onImageVisible"
          />
          <div class="movie-overlay">
            <button 
              class="action-button add-button" 
              @click.stop="toggleWatchlist(movie)"
              :title="isMovieInWatchlist(movie.id) ? 'Remove from Watchlist' : 'Add to Watchlist'"
              :class="{ 'in-watchlist': isMovieInWatchlist(movie.id) }"
            >
              <svg v-if="!isMovieInWatchlist(movie.id)" width="20" height="20" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M12 5V19M5 12H19" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
              <svg v-else width="20" height="20" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M9 12L11 14L15 10M21 12C21 16.9706 16.9706 21 12 21C7.02944 21 3 16.9706 3 12C3 7.02944 7.02944 3 12 3C16.9706 3 21 7.02944 21 12Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
            </button>

          </div>
          <div class="rating-badge" v-if="movie.vote_average">
            <svg width="12" height="12" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M12 2L15.09 8.26L22 9.27L17 14.14L18.18 21.02L12 17.77L5.82 21.02L7 14.14L2 9.27L8.91 8.26L12 2Z" fill="currentColor"/>
            </svg>
            <span>{{ movie.vote_average.toFixed(1) }}</span>
          </div>
        </div>
        
        <div class="movie-content">
          <h3 class="movie-title">{{ movie.title }}</h3>
          <p class="movie-year" v-if="movie.release_date">
            {{ new Date(movie.release_date).getFullYear() }}
          </p>
        </div>
      </div>
    </div>

    <!-- Empty state -->
    <div v-if="!loading && movies.length === 0" class="empty-state">
      <svg width="64" height="64" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path d="M7 4V2C7 1.44772 7.44772 1 8 1H16C16.5523 1 17 1.44772 17 2V4M7 4H5C3.89543 4 3 4.89543 3 6V20C3 21.1046 3.89543 22 5 22H19C20.1046 22 21 21.1046 21 20V6C21 4.89543 20.1046 4 19 4H17M7 4H17M9 9H15M9 13H15M9 17H15" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>
      <h3>No movies found</h3>
      <p>Try adjusting your search criteria</p>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref, onMounted, watch } from 'vue';
import { useRouter } from 'vue-router';
import { invoke } from "@tauri-apps/api/core";
import { fetchMovies } from '../../services/tmdbService';
import { defineProps } from 'vue';
import { useWatchlistStore } from '../../stores/watchlistStore';
import { useToast } from '../../composables/useToast';
import LazyImage from '../LazyImage.vue';

const props = defineProps<{ searchedMovies: any[] }>();

const movies = ref<any[]>(props.searchedMovies || []);
const loading = ref(true);

const router = useRouter();
const { isMovieInWatchlist, addMovieToWatchlist, removeMovieFromWatchlist } = useWatchlistStore();
const { success, error } = useToast();

// Performance optimization: preload images that are likely to be viewed next
const preloadedImages = ref(new Set<string>());

// Image event handlers
const onImageLoad = (event: Event) => {
  // Optional: handle successful image loads
};

const onImageError = (event: Event) => {
  // Optional: handle image load errors
};

const onImageVisible = () => {
  // Optional: handle when image becomes visible
};

// Preload next few images for better perceived performance
const preloadNextImages = (currentIndex: number) => {
  const nextImages = movies.value.slice(currentIndex + 1, currentIndex + 4);
  nextImages.forEach(movie => {
    if (!preloadedImages.value.has(movie.poster_url)) {
      const img = new Image();
      img.src = movie.poster_url;
      preloadedImages.value.add(movie.poster_url);
    }
  });
};

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

const toggleWatchlist = async (movie: any) => {
  try {
    if (isMovieInWatchlist(movie.id)) {
      await removeMovieFromWatchlist(movie);
      success('Removed from Watchlist', `${movie.title} has been removed from your watchlist`);
    } else {
      await addMovieToWatchlist(movie);
      success('Added to Watchlist', `${movie.title} has been added to your watchlist`);
    }
  } catch (err) {
    console.error('Failed to update watchlist:', err);
    error('Watchlist Error', 'Failed to update your watchlist. Please try again.');
  }
};

const goToMovieDetails = (movie: any) => {
  localStorage.setItem('selectedMovie', JSON.stringify(movie));
  localStorage.setItem('movieNavigationContext', JSON.stringify({
    from: 'popular',
    tab: 'movies'
  }));
  router.push({
    name: 'MovieDetails',
    params: { id: movie.id }
  });
};

onMounted(() => {
  if (movies.value.length === 0) {
    loadMovies().then(() => {
      // Preload first few images immediately
      if (movies.value.length > 0) {
        preloadNextImages(-1);
      }
    });
  } else {
    loading.value = false;
    // Preload first few images
    preloadNextImages(-1);
  }
});

watch(() => props.searchedMovies, (newMovies) => {
  if (newMovies && newMovies.length > 0) {
    movies.value = newMovies;
    loading.value = false;
    // Clear previous preloaded images and preload new ones
    preloadedImages.value.clear();
    preloadNextImages(-1);
  }
});
</script>

<style scoped>
.movie-grid-container {
  padding: var(--spacing-lg);
  max-width: 1400px;
  margin: 0 auto;
}

.movie-grid {
  display: grid;
  gap: var(--spacing-lg);
  grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
  /* Performance optimizations */
  will-change: contents;
  contain: layout style;
  transform: translateZ(0); /* Force hardware acceleration */
}

/* Movie Card Styles */
.movie-card {
  background: var(--color-card-background);
  border: 1px solid var(--color-card-border);
  border-radius: var(--radius-large);
  overflow: hidden;
  cursor: pointer;
  transition: transform var(--transition-medium), box-shadow var(--transition-medium), border-color var(--transition-fast);
  box-shadow: var(--shadow-small);
  position: relative;
  /* Performance optimizations */
  will-change: transform, box-shadow;
  contain: layout style paint;
  backface-visibility: hidden;
}

.movie-card:hover {
  transform: translateY(-4px);
  box-shadow: var(--shadow-large);
  border-color: var(--color-accent-primary);
}

.movie-card:hover .movie-content {
  background: linear-gradient(135deg, var(--color-card-background) 0%, var(--color-surface) 100%);
}

.movie-card:active {
  transform: translateY(-2px);
  transition: transform var(--transition-fast);
}

.movie-poster-container {
  position: relative;
  overflow: hidden;
  aspect-ratio: 2/3;
}

.movie-poster {
  transition: transform var(--transition-medium);
  /* Performance optimizations */
  will-change: transform;
  backface-visibility: hidden;
}

.movie-card:hover .movie-poster {
  transform: scale(1.05);
}

.movie-overlay {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: linear-gradient(
    to bottom,
    rgba(0, 0, 0, 0) 0%,
    rgba(0, 0, 0, 0.1) 50%,
    rgba(0, 0, 0, 0.8) 100%
  );
  display: flex;
  align-items: flex-end;
  justify-content: flex-end;
  padding: var(--spacing-md);
  opacity: 0;
  transition: opacity var(--transition-medium);
}

.movie-card:hover .movie-overlay {
  opacity: 1;
}

.action-button {
  background: rgba(255, 255, 255, 0.1);
  backdrop-filter: blur(8px);
  border: 1px solid rgba(255, 255, 255, 0.2);
  color: white;
  width: 44px;
  height: 44px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all var(--transition-fast);
  min-height: auto;
}

.action-button:hover {
  background: rgba(255, 255, 255, 0.2);
  border-color: rgba(255, 255, 255, 0.4);
  transform: scale(1.1);
}

.add-button:hover {
  background: var(--color-accent-primary);
  border-color: var(--color-accent-primary);
}

.add-button.in-watchlist {
  background: var(--color-success);
  border-color: var(--color-success);
}

.add-button.in-watchlist:hover {
  background: var(--color-error);
  border-color: var(--color-error);
}



.rating-badge {
  position: absolute;
  top: var(--spacing-sm);
  right: var(--spacing-sm);
  background: rgba(0, 0, 0, 0.8);
  backdrop-filter: blur(8px);
  color: #fbbf24;
  padding: var(--spacing-xs) var(--spacing-sm);
  border-radius: var(--radius-medium);
  font-size: 0.75rem;
  font-weight: 600;
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  border: 1px solid rgba(251, 191, 36, 0.3);
}

.movie-content {
  padding: var(--spacing-md);
  transition: background var(--transition-medium);
  position: relative;
}

.movie-title {
  font-size: 1rem;
  font-weight: 600;
  color: var(--color-text-primary);
  margin: 0 0 var(--spacing-xs) 0;
  line-height: 1.4;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
  transition: color var(--transition-fast);
}

.movie-card:hover .movie-title {
  color: var(--color-accent-primary);
}

.movie-year {
  font-size: 0.875rem;
  color: var(--color-text-secondary);
  margin: 0;
}

/* Skeleton Loader Styles */
.skeleton-card {
  background: var(--color-card-background);
  border: 1px solid var(--color-card-border);
  border-radius: var(--radius-large);
  overflow: hidden;
}

.skeleton-poster {
  aspect-ratio: 2/3;
  background: var(--color-surface);
}

.skeleton-content {
  padding: var(--spacing-md);
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
}

.skeleton-title {
  height: 20px;
  background: var(--color-surface);
  border-radius: var(--radius-small);
  width: 80%;
}

.skeleton-subtitle {
  height: 16px;
  background: var(--color-surface);
  border-radius: var(--radius-small);
  width: 40%;
}

/* Empty State */
.empty-state {
  grid-column: 1 / -1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: var(--spacing-2xl);
  text-align: center;
  color: var(--color-text-secondary);
}

.empty-state svg {
  color: var(--color-text-muted);
  margin-bottom: var(--spacing-md);
}

.empty-state h3 {
  color: var(--color-text-primary);
  margin-bottom: var(--spacing-sm);
}

.empty-state p {
  margin: 0;
}

/* Performance optimizations for reduced motion */
@media (prefers-reduced-motion: reduce) {
  .movie-card {
    transition: none;
  }
  
  .movie-poster {
    transition: none;
  }
  
  .movie-card:hover {
    transform: none;
  }
  
  .movie-card:hover .movie-poster {
    transform: none;
  }
}

/* Responsive Design */
@media (min-width: 1600px) {
  .movie-grid {
    grid-template-columns: repeat(8, 1fr);
  }
}

@media (min-width: 1200px) and (max-width: 1599px) {
  .movie-grid {
    grid-template-columns: repeat(6, 1fr);
  }
}

@media (min-width: 768px) and (max-width: 1199px) {
  .movie-grid {
    grid-template-columns: repeat(4, 1fr);
  }
}

@media (min-width: 480px) and (max-width: 767px) {
  .movie-grid {
    grid-template-columns: repeat(3, 1fr);
    gap: var(--spacing-md);
  }
  
  .movie-grid-container {
    padding: var(--spacing-md);
  }
}

@media (max-width: 479px) {
  .movie-grid {
    grid-template-columns: repeat(2, 1fr);
    gap: var(--spacing-sm);
  }
  
  .movie-grid-container {
    padding: var(--spacing-sm);
  }
  
  .movie-content {
    padding: var(--spacing-sm);
  }
  
  .movie-title {
    font-size: 0.875rem;
  }
  
  .movie-year {
    font-size: 0.75rem;
  }
  
  .action-button {
    width: 36px;
    height: 36px;
  }
  
  .action-button svg {
    width: 16px;
    height: 16px;
  }
}

/* GPU acceleration for better performance */
@supports (transform: translateZ(0)) {
  .movie-card {
    transform: translateZ(0);
  }
  
  .movie-overlay {
    transform: translateZ(0);
  }
}

/* Container queries for better responsive behavior */
@supports (container-type: inline-size) {
  .movie-grid-container {
    container-type: inline-size;
  }
  
  @container (max-width: 600px) {
    .movie-grid {
      grid-template-columns: repeat(2, 1fr);
      gap: var(--spacing-sm);
    }
  }
  
  @container (min-width: 1400px) {
    .movie-grid {
      grid-template-columns: repeat(7, 1fr);
    }
  }
}
</style>