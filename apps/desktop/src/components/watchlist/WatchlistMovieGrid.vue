<template>
  <div class="movie-grid-container">
    <!-- Skeleton loaders -->
    <div v-if="isLoadingMovies" class="movie-grid">
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
        v-for="movie in filteredMovies" 
        :key="movie.id" 
        class="movie-card"
        @click="goToMovieDetails(movie)"
      >
        <div class="movie-poster-container">
          <img 
            :src="movie.poster_url" 
            :alt="movie.title" 
            class="movie-poster"
            loading="lazy"
          />
          <div class="movie-overlay">
            <button 
              class="action-button remove-button" 
              @click.stop="removeFromWatchlist(movie)"
              title="Remove from Watchlist"
            >
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M19 7L18.1327 19.1425C18.0579 20.1891 17.187 21 16.1378 21H7.86224C6.81296 21 5.94208 20.1891 5.86732 19.1425L5 7M10 11V17M14 11V17M15 7V4C15 3.44772 14.5523 3 14 3H10C9.44772 3 9 3.44772 9 4V7M4 7H20" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
            </button>

          </div>
          <div class="rating-badge" v-if="movie.vote_average">
            <svg width="12" height="12" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M12 2L15.09 8.26L22 9.27L17 14.14L18.18 21.02L12 17.77L5.82 21.02L7 14.14L2 9.27L8.91 8.26L12 2Z" fill="currentColor"/>
            </svg>
            <span>{{ movie.vote_average.toFixed(1) }}</span>
          </div>
          <div class="watchlist-badge">
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M19 14C19 18.4183 15.4183 22 11 22C6.58172 22 3 18.4183 3 14C3 9.58172 6.58172 6 11 6C15.4183 6 19 9.58172 19 14Z" stroke="currentColor" stroke-width="2"/>
              <path d="M9 11L11 13L15 9" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
            <span>Saved</span>
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
    <div v-if="!isLoadingMovies && filteredMovies.length === 0" class="empty-state">
      <svg width="64" height="64" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path d="M19 14C19 18.4183 15.4183 22 11 22C6.58172 22 3 18.4183 3 14C3 9.58172 6.58172 6 11 6C15.4183 6 19 9.58172 19 14Z" stroke="currentColor" stroke-width="2"/>
        <path d="M9 11L11 13L15 9" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>
      <h3>Your watchlist is empty</h3>
      <p>Add some movies to get started</p>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref, computed, onMounted, watch } from 'vue';
import { invoke } from "@tauri-apps/api/core";
import { defineProps } from 'vue';
import { fetchMovieWatchlist } from '../../services/tmdbService';
import { useWatchlistStore } from '../../stores/watchlistStore';
import { useToast } from '../../composables/useToast';
import { useRouter } from 'vue-router';

const props = defineProps<{ searchQuery: string }>();

const { watchlistMovies, isLoadingMovies, removeMovieFromWatchlist } = useWatchlistStore();
const { success, error } = useToast();
const router = useRouter();

const removeFromWatchlist = async (movie: any) => {
  try {
    await removeMovieFromWatchlist(movie);
    success('Removed from Watchlist', `${movie.title} has been removed from your watchlist`);
  } catch (err) {
    console.error('Failed to remove movie from watchlist:', err);
    error('Watchlist Error', 'Failed to remove movie from your watchlist. Please try again.');
  }
};

const goToMovieDetails = (movie: any) => {
  localStorage.setItem('selectedMovie', JSON.stringify(movie));
  router.push({
    name: 'MovieDetails',
    params: { id: movie.id }
  });
};



const filteredMovies = computed(() => {
  if (!props.searchQuery) {
    return watchlistMovies.value;
  }
  return watchlistMovies.value.filter(movie =>
    movie.title.toLowerCase().includes(props.searchQuery.toLowerCase())
  );
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
}

/* Movie Card Styles */
.movie-card {
  background: var(--color-card-background);
  border: 1px solid var(--color-card-border);
  border-radius: var(--radius-large);
  overflow: hidden;
  cursor: pointer;
  transition: all var(--transition-medium);
  box-shadow: var(--shadow-small);
  position: relative;
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
  width: 100%;
  height: 100%;
  object-fit: cover;
  transition: transform var(--transition-medium);
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

.remove-button:hover {
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

.watchlist-badge {
  position: absolute;
  top: var(--spacing-sm);
  left: var(--spacing-sm);
  background: rgba(35, 134, 54, 0.9);
  backdrop-filter: blur(8px);
  color: white;
  padding: var(--spacing-xs) var(--spacing-sm);
  border-radius: var(--radius-medium);
  font-size: 0.75rem;
  font-weight: 600;
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  border: 1px solid rgba(35, 134, 54, 0.3);
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
</style>
