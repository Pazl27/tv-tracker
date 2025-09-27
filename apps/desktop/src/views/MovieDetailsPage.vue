<template>
  <div class="movie-details-page">
    <!-- Loading state -->
    <div v-if="!movie" class="loading-container">
      <div class="loading-skeleton">
        <div class="skeleton-poster animate-shimmer"></div>
        <div class="skeleton-info">
          <div class="skeleton-title animate-shimmer"></div>
          <div class="skeleton-meta animate-shimmer"></div>
          <div class="skeleton-overview animate-shimmer"></div>
          <div class="skeleton-overview animate-shimmer"></div>
          <div class="skeleton-overview animate-shimmer"></div>
        </div>
      </div>
    </div>

    <!-- Movie details content -->
    <div v-else class="movie-details-container">
      <!-- Background -->
      <div class="movie-background">
        <img :src="movie.poster_url" :alt="movie.title" class="background-image" />
        <div class="background-overlay"></div>
      </div>

      <!-- Navigation -->
      <nav class="movie-nav">
        <button @click="goBack" class="back-button">
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M19 12H5M5 12L12 19M5 12L12 5" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
          <span>Back</span>
        </button>
      </nav>

      <!-- Movie content -->
      <div class="movie-content">
        <div class="movie-poster-section">
          <div class="poster-container">
            <img :src="movie.poster_url" :alt="movie.title" class="movie-poster" />
            <div class="poster-actions">
              <button
                class="action-btn primary"
                @click="toggleWatchlist"
                :class="{ 'in-watchlist': movie && isMovieInWatchlist(movie.id) }"
              >
                <svg v-if="!movie || !isMovieInWatchlist(movie.id)" width="20" height="20" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                  <path d="M12 5V19M5 12H19" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                </svg>
                <svg v-else width="20" height="20" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                  <path d="M9 12L11 14L15 10M21 12C21 16.9706 16.9706 21 12 21C7.02944 21 3 16.9706 3 12C3 7.02944 7.02944 3 12 3C16.9706 3 21 7.02944 21 12Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                </svg>
                {{ movie && isMovieInWatchlist(movie.id) ? 'Remove from Watchlist' : 'Add to Watchlist' }}
              </button>
              <button class="action-btn secondary" @click="shareMovie">
                <svg width="20" height="20" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                  <path d="M4 12V20C4 20.5523 4.44772 21 5 21H19C19.5523 21 20 20.5523 20 20V12M16 6L12 2L8 6M12 2V15" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                </svg>
                Share
              </button>
            </div>
          </div>
        </div>

        <div class="movie-info-section">
          <!-- Title and basic info -->
          <div class="movie-header">
            <h1 class="movie-title">{{ movie.title }}</h1>
            <div class="movie-meta">
              <div class="meta-item">
                <span class="meta-label">Year</span>
                <span class="meta-value">{{ new Date(movie.release_date).getFullYear() }}</span>
              </div>
              <div class="meta-separator">•</div>
              <div class="meta-item">
                <span class="meta-label">Runtime</span>
                <span class="meta-value">{{ movie.runtime }} min</span>
              </div>
              <div class="meta-separator">•</div>
              <div class="rating-container">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                  <path d="M12 2L15.09 8.26L22 9.27L17 14.14L18.18 21.02L12 17.77L5.82 21.02L7 14.14L2 9.27L8.91 8.26L12 2Z" fill="currentColor"/>
                </svg>
                <span class="rating-value">{{ movie.vote_average.toFixed(1) }}</span>
                <span class="rating-max">/10</span>
              </div>
            </div>
          </div>

          <!-- Genres -->
          <div class="genres-section" v-if="movie.genres && movie.genres.length">
            <h3 class="section-title">Genres</h3>
            <div class="genres-list">
              <span v-for="genre in movie.genres" :key="genre" class="genre-tag">
                {{ genre }}
              </span>
            </div>
          </div>

          <!-- Rating Section -->
          <div class="rating-section">
            <h3 class="section-title">Your Rating</h3>
            <div v-if="currentRating > 0" class="current-rating">
              <StarRating
                :model-value="currentRating"
                :readonly="true"
                :show-rating-text="true"
                :show-clear-button="false"
                size="medium"
              />
              <div class="rating-actions">
                <button @click="openRatingPopup" class="rating-btn edit">
                  <svg width="16" height="16" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path d="M11 4H4C3.46957 4 2.96086 4.21071 2.58579 4.58579C2.21071 4.96086 2 5.46957 2 6V20C2 20.5304 2.21071 21.0391 2.58579 21.4142C2.96086 21.7893 3.46957 22 4 22H18C18.5304 22 19.0391 21.7893 19.4142 21.4142C19.7893 21.0391 20 20.5304 20 20V13" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                    <path d="M18.5 2.5C18.8978 2.10217 19.4374 1.87868 20 1.87868C20.5626 1.87868 21.1022 2.10217 21.5 2.5C21.8978 2.89783 22.1213 3.43739 22.1213 4C22.1213 4.56261 21.8978 5.10217 21.5 5.5L12 15L8 16L9 12L18.5 2.5Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                  </svg>
                  Edit Rating
                </button>
                <button @click="handleRatingClear" class="rating-btn remove">
                  <svg width="16" height="16" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path d="M18 6L6 18M6 6L18 18" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                  </svg>
                  Remove
                </button>
              </div>
            </div>
            <div v-else class="no-rating">
              <p>You haven't rated this movie yet</p>
              <button @click="openRatingPopup" class="rating-btn primary">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                  <path d="M12 2L15.09 8.26L22 9.27L17 14.14L18.18 21.02L12 17.77L5.82 21.02L7 14.14L2 9.27L8.91 8.26L12 2Z" fill="currentColor"/>
                </svg>
                Rate This Movie
              </button>
            </div>
          </div>

          <!-- Overview -->
          <div class="overview-section">
            <h3 class="section-title">Overview</h3>
            <p class="overview-text">{{ movie.overview }}</p>
          </div>

          <!-- Additional details -->
          <div class="details-grid">
            <div class="detail-item" v-if="movie.release_date">
              <span class="detail-label">Release Date</span>
              <span class="detail-value">{{ formatDate(movie.release_date) }}</span>
            </div>
            <div class="detail-item" v-if="movie.runtime">
              <span class="detail-label">Duration</span>
              <span class="detail-value">{{ formatRuntime(movie.runtime) }}</span>
            </div>
            <div class="detail-item" v-if="movie.vote_count">
              <span class="detail-label">Vote Count</span>
              <span class="detail-value">{{ movie.vote_count.toLocaleString() }}</span>
            </div>
            <div class="detail-item" v-if="movie.popularity">
              <span class="detail-label">Popularity</span>
              <span class="detail-value">{{ Math.round(movie.popularity) }}</span>
            </div>
          </div>
        </div>

        <!-- Notes Section -->
        <NotesSection
          v-if="movie"
          :content-id="movie.id"
          content-type="movie"
          :content-title="movie.title"
        />
      </div>
    </div>

    <!-- No data state -->
    <div v-if="!movie && !loading" class="no-data-container">
      <div class="no-data-content">
        <svg width="64" height="64" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
          <path d="M12 9V13M12 17H12.01M21 12C21 16.9706 16.9706 21 12 21C7.02944 21 3 16.9706 3 12C3 7.02944 7.02944 3 12 3C16.9706 3 21 7.02944 21 12Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
        <h3>Movie Not Found</h3>
        <p>The movie data could not be loaded. Please try again.</p>
        <button @click="goBack" class="btn-primary">Go Back</button>
      </div>
    </div>

    <!-- Rating Popup -->
    <RatingPopup
      :is-visible="showRatingPopup"
      :content="movie"
      content-type="Movie"
      :existing-rating="currentRating"
      :existing-watched-at="existingWatchedAt"
      @close="closeRatingPopup"
      @save="handleRatingSave"
    />
  </div>
</template>

<script setup lang="ts">
import NotesSection from '../components/NotesSection.vue';
import RatingPopup from '../components/RatingPopup.vue';
import StarRating from '../components/StarRating.vue';
import { useToast } from '../composables/useToast';
import { getMovieDetails } from '../services/tmdbService';
import { useRatingStore } from '../stores/ratingStore';
import { useWatchlistStore } from '../stores/watchlistStore';
import { invoke } from '@tauri-apps/api/core';
import { ref, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';

const movie = ref<any>(null);
const loading = ref(true);
const currentRating = ref(0);
const showRatingPopup = ref(false);
const existingWatchedAt = ref('');
const route = useRoute();
const router = useRouter();
const { isMovieInWatchlist, addMovieToWatchlist, removeMovieFromWatchlist } = useWatchlistStore();
const { getMovieRating, rateMovie, removeMovieRating } = useRatingStore();
const { success, error } = useToast();

const fetchMovie = async (movieId: number) => {
  try {
    loading.value = true;
    movie.value = await getMovieDetails(invoke, movieId);

    // Load existing rating if any
    const existingRating = getMovieRating.value(movieId);
    currentRating.value = existingRating;
  } catch (error) {
    console.error('Failed to load movie details:', error);
  } finally {
    loading.value = false;
  }
};

const goBack = () => {
  // Get navigation context from localStorage
  const contextStr = localStorage.getItem('movieNavigationContext');

  if (contextStr) {
    try {
      const context = JSON.parse(contextStr);

      if (context.from === 'watchlist') {
        router.push({
          path: '/watchlist',
          query: { tab: 'movies' }
        });
      } else if (context.from === 'popular') {
        router.push({
          path: '/popular',
          query: { tab: 'movies' }
        });
      } else {
        router.go(-1);
      }

      // Clean up the context after use
      localStorage.removeItem('movieNavigationContext');
    } catch (error) {
      console.error('Error parsing navigation context:', error);
      router.go(-1);
    }
  } else {
    router.go(-1);
  }
};

const toggleWatchlist = async () => {
  if (!movie.value) return;

  try {
    if (isMovieInWatchlist(movie.value.id)) {
      await removeMovieFromWatchlist(movie.value);
      success('Removed from Watchlist', `${movie.value.title} has been removed from your watchlist`);
    } else {
      await addMovieToWatchlist(movie.value);
      success('Added to Watchlist', `${movie.value.title} has been added to your watchlist`);
    }
  } catch (err) {
    console.error('Failed to update watchlist:', err);
    error('Watchlist Error', 'Failed to update your watchlist. Please try again.');
  }
};

const shareMovie = () => {
  if (!movie.value) return;

  if (navigator.share) {
    navigator.share({
      title: movie.value.title,
      text: movie.value.overview,
      url: window.location.href,
    });
  } else {
    // Fallback: copy to clipboard
    navigator.clipboard.writeText(window.location.href);
    // TODO: Add toast notification
  }
};

const formatDate = (dateString: string) => {
  const date = new Date(dateString);
  return date.toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'long',
    day: 'numeric'
  });
};

const formatRuntime = (minutes: number) => {
  const hours = Math.floor(minutes / 60);
  const mins = minutes % 60;
  return `${hours}h ${mins}m`;
};

const openRatingPopup = () => {
  showRatingPopup.value = true;
};

const closeRatingPopup = () => {
  showRatingPopup.value = false;
};

const handleRatingSave = async (data: { rating: number; watchedAt: string }) => {
  if (!movie.value) return;

  try {
    await rateMovie(movie.value, data.rating, data.watchedAt);
    currentRating.value = data.rating;
    existingWatchedAt.value = data.watchedAt;
    success('Movie Rated', `You rated ${movie.value.title} ${data.rating}/5 stars`);
    closeRatingPopup();
  } catch (err) {
    console.error('Failed to rate movie:', err);
    error('Rating Error', 'Failed to save your rating. Please try again.');
  }
};

const handleRatingClear = async () => {
  if (!movie.value) return;

  try {
    await removeMovieRating(movie.value.id);
    currentRating.value = 0;
    success('Rating Removed', `Removed your rating for ${movie.value.title}`);
  } catch (err) {
    console.error('Failed to remove movie rating:', err);
    error('Rating Error', 'Failed to remove your rating. Please try again.');
  }
};

onMounted(() => {
  const movieId = route.params.id;
  if (movieId) {
    fetchMovie(Number(movieId));
  } else {
    console.error('Movie ID is missing. Ensure it is passed via router state.');
    loading.value = false;
  }
});
</script>

<style scoped>
.movie-details-page {
  min-height: 100vh;
  background: var(--color-background);
}

/* Loading State */
.loading-container {
  padding: var(--spacing-lg);
  max-width: 1200px;
  margin: 0 auto;
}

.loading-skeleton {
  display: grid;
  grid-template-columns: 300px 1fr;
  gap: var(--spacing-2xl);
  margin-top: var(--spacing-2xl);
}

.skeleton-poster {
  aspect-ratio: 2/3;
  background: var(--color-surface);
  border-radius: var(--radius-large);
}

.skeleton-info {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-lg);
}

.skeleton-title {
  height: 40px;
  background: var(--color-surface);
  border-radius: var(--radius-medium);
  width: 60%;
}

.skeleton-meta {
  height: 24px;
  background: var(--color-surface);
  border-radius: var(--radius-medium);
  width: 40%;
}

.skeleton-overview {
  height: 20px;
  background: var(--color-surface);
  border-radius: var(--radius-medium);
}

.skeleton-overview:last-child {
  width: 70%;
}

/* Movie Details */
.movie-details-container {
  position: relative;
  min-height: 100vh;
}

.movie-background {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  z-index: -1;
}

.background-image {
  width: 100%;
  height: 100%;
  object-fit: cover;
  filter: blur(20px) brightness(0.3);
  transform: scale(1.1);
}

.background-overlay {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: linear-gradient(
    to bottom,
    rgba(13, 17, 23, 0.9) 0%,
    rgba(13, 17, 23, 0.95) 50%,
    rgba(13, 17, 23, 0.98) 100%
  );
}

.movie-nav {
  position: sticky;
  top: 0;
  z-index: 100;
  padding: var(--spacing-lg);
  background: rgba(13, 17, 23, 0.8);
  backdrop-filter: blur(8px);
  border-bottom: 1px solid var(--color-border);
}

.back-button {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  color: var(--color-text-primary);
  padding: var(--spacing-sm) var(--spacing-md);
  border-radius: var(--radius-medium);
  cursor: pointer;
  transition: all var(--transition-fast);
  font-weight: 500;
  min-height: auto;
}

.back-button:hover {
  background: var(--color-surface-hover);
  border-color: var(--color-accent-primary);
  color: var(--color-accent-primary);
}

.movie-content {
  display: grid;
  grid-template-columns: 300px 1fr;
  gap: var(--spacing-2xl);
  max-width: 1200px;
  margin: 0 auto;
  padding: var(--spacing-2xl) var(--spacing-lg);
  position: relative;
  z-index: 1;
}

/* Poster Section */
.poster-container {
  position: sticky;
  top: 120px;
}

.movie-poster {
  width: 100%;
  aspect-ratio: 2/3;
  object-fit: cover;
  border-radius: var(--radius-large);
  box-shadow: var(--shadow-large);
  border: 1px solid var(--color-border);
}

.poster-actions {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-md);
  margin-top: var(--spacing-lg);
}

.action-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-md) var(--spacing-lg);
  border-radius: var(--radius-medium);
  font-weight: 500;
  cursor: pointer;
  transition: all var(--transition-fast);
  border: none;
  min-height: auto;
}

.action-btn.primary {
  background: var(--color-accent-primary);
  color: white;
}

.action-btn.primary:hover {
  background: var(--color-accent-primary-hover);
  transform: translateY(-1px);
}

.action-btn.primary.in-watchlist {
  background: var(--color-success);
}

.action-btn.primary.in-watchlist:hover {
  background: var(--color-error);
}

.action-btn.secondary {
  background: var(--color-surface);
  color: var(--color-text-primary);
  border: 1px solid var(--color-border);
}

.action-btn.secondary:hover {
  background: var(--color-surface-hover);
  border-color: var(--color-accent-secondary);
  color: var(--color-accent-secondary);
}

/* Movie Info Section */
.movie-info-section {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-2xl);
}

.movie-header {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-md);
}

.movie-title {
  font-size: 3rem;
  font-weight: 700;
  color: var(--color-text-primary);
  line-height: 1.2;
  margin: 0;
}

.movie-meta {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
  flex-wrap: wrap;
}

.meta-item {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.meta-label {
  font-size: 0.75rem;
  color: var(--color-text-muted);
  text-transform: uppercase;
  font-weight: 500;
  letter-spacing: 0.5px;
}

.meta-value {
  font-size: 1rem;
  color: var(--color-text-primary);
  font-weight: 500;
}

.meta-separator {
  color: var(--color-text-muted);
  font-size: 1.25rem;
}

.rating-container {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  color: #fbbf24;
}

.rating-value {
  font-weight: 600;
  color: var(--color-text-primary);
}

.rating-max {
  color: var(--color-text-muted);
}

.section-title {
  font-size: 1.5rem;
  font-weight: 600;
  color: var(--color-text-primary);
  margin: 0 0 var(--spacing-md) 0;
}

.genres-list {
  display: flex;
  flex-wrap: wrap;
  gap: var(--spacing-sm);
}

.genre-tag {
  background: var(--color-surface);
  color: var(--color-text-primary);
  padding: var(--spacing-xs) var(--spacing-md);
  border-radius: var(--radius-xl);
  font-size: 0.875rem;
  font-weight: 500;
  border: 1px solid var(--color-border);
}

.overview-text {
  font-size: 1.125rem;
  line-height: 1.7;
  color: var(--color-text-secondary);
  margin: 0;
}

.rating-section {
  padding: var(--spacing-xl);
  background: var(--color-surface);
  border-radius: var(--radius-large);
  border: 1px solid var(--color-border);
  margin-bottom: var(--spacing-xl);
}

.rating-section .section-title {
  margin-bottom: var(--spacing-lg);
}

.current-rating {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--spacing-lg);
}

.rating-actions {
  display: flex;
  gap: var(--spacing-sm);
}

.no-rating {
  text-align: center;
  padding: var(--spacing-xl) 0;
}

.no-rating p {
  margin: 0 0 var(--spacing-lg) 0;
  color: var(--color-text-secondary);
}

.rating-btn {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  padding: var(--spacing-sm) var(--spacing-md);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-medium);
  background: var(--color-surface);
  color: var(--color-text-secondary);
  cursor: pointer;
  transition: all 0.2s ease;
  font-size: 0.875rem;
  font-weight: 500;
}

.rating-btn:hover {
  background: var(--color-background);
  color: var(--color-text-primary);
}

.rating-btn.primary {
  background: var(--color-primary);
  color: white;
  border-color: var(--color-primary);
}

.rating-btn.primary:hover {
  background: var(--color-primary-dark);
}

.rating-btn.edit:hover {
  background: var(--color-primary);
  color: white;
  border-color: var(--color-primary);
}

.rating-btn.remove:hover {
  background: #ef4444;
  color: white;
  border-color: #ef4444;
}

.details-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: var(--spacing-lg);
  padding: var(--spacing-xl);
  background: var(--color-surface);
  border-radius: var(--radius-large);
  border: 1px solid var(--color-border);
}

.detail-item {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.detail-label {
  font-size: 0.875rem;
  color: var(--color-text-muted);
  font-weight: 500;
}

.detail-value {
  font-size: 1.125rem;
  color: var(--color-text-primary);
  font-weight: 600;
}

/* No Data State */
.no-data-container {
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 100vh;
  padding: var(--spacing-lg);
}

.no-data-content {
  text-align: center;
  max-width: 400px;
}

.no-data-content svg {
  color: var(--color-text-muted);
  margin-bottom: var(--spacing-lg);
}

.no-data-content h3 {
  color: var(--color-text-primary);
  margin-bottom: var(--spacing-md);
}

.no-data-content p {
  color: var(--color-text-secondary);
  margin-bottom: var(--spacing-lg);
}

/* Responsive Design */
@media (max-width: 1024px) {
  .movie-content {
    grid-template-columns: 250px 1fr;
    gap: var(--spacing-xl);
  }

  .movie-title {
    font-size: 2.5rem;
  }
}

@media (max-width: 768px) {
  .movie-content {
    grid-template-columns: 1fr;
    gap: var(--spacing-lg);
    padding: var(--spacing-lg);
  }

  .poster-container {
    position: static;
    max-width: 300px;
    margin: 0 auto;
  }

  .movie-title {
    font-size: 2rem;
    text-align: center;
  }

  .movie-meta {
    justify-content: center;
  }

  .details-grid {
    grid-template-columns: 1fr;
  }

  .loading-skeleton {
    grid-template-columns: 1fr;
    gap: var(--spacing-lg);
  }

  .skeleton-poster {
    max-width: 300px;
    margin: 0 auto;
  }
}

@media (max-width: 480px) {
  .movie-nav {
    padding: var(--spacing-md);
  }

  .movie-content {
    padding: var(--spacing-md);
  }

  .movie-title {
    font-size: 1.75rem;
  }

  .overview-text {
    font-size: 1rem;
  }

  .poster-actions {
    flex-direction: row;
  }

  .action-btn {
    flex: 1;
    padding: var(--spacing-sm) var(--spacing-md);
    font-size: 0.875rem;
  }
}
</style>
