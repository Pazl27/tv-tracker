<template>
  <div class="tvshow-details-page">
    <!-- Loading state -->
    <div v-if="!tvShow" class="loading-container">
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

    <!-- TV Show details content -->
    <div v-else class="tvshow-details-container">
      <!-- Background -->
      <div class="tvshow-background">
        <img :src="tvShow.poster_url" :alt="tvShow.name" class="background-image" />
        <div class="background-overlay"></div>
      </div>

      <!-- Navigation -->
      <nav class="tvshow-nav">
        <button @click="goBack" class="back-button">
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M19 12H5M5 12L12 19M5 12L12 5" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
          <span>Back</span>
        </button>
      </nav>

      <!-- TV Show content -->
      <div class="tvshow-content">
        <div class="tvshow-poster-section">
          <div class="poster-container">
            <img :src="tvShow.poster_url" :alt="tvShow.name" class="tvshow-poster" />
            <div class="poster-actions">
              <button
                class="action-btn primary"
                @click="toggleWatchlist"
                :class="{ 'in-watchlist': tvShow && isTvShowInWatchlist(tvShow.id) }"
              >
                <svg v-if="!tvShow || !isTvShowInWatchlist(tvShow.id)" width="20" height="20" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                  <path d="M12 5V19M5 12H19" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                </svg>
                <svg v-else width="20" height="20" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                  <path d="M9 12L11 14L15 10M21 12C21 16.9706 16.9706 21 12 21C7.02944 21 3 16.9706 3 12C3 7.02944 7.02944 3 12 3C16.9706 3 21 7.02944 21 12Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                </svg>
                {{ tvShow && isTvShowInWatchlist(tvShow.id) ? 'Remove from Watchlist' : 'Add to Watchlist' }}
              </button>
              <button class="action-btn secondary" @click="shareShow">
                <svg width="20" height="20" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                  <path d="M4 12V20C4 20.5523 4.44772 21 5 21H19C19.5523 21 20 20.5523 20 20V12M16 6L12 2L8 6M12 2V15" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                </svg>
                Share
              </button>
            </div>
          </div>
        </div>

        <div class="tvshow-info-section">
          <!-- Title and basic info -->
          <div class="tvshow-header">
            <h1 class="tvshow-title">{{ tvShow.name }}</h1>
            <div class="tvshow-meta">
              <div class="meta-item">
                <span class="meta-label">First Aired</span>
                <span class="meta-value">{{ new Date(tvShow.first_air_date).getFullYear() }}</span>
              </div>
              <div class="meta-separator">•</div>
              <div class="meta-item" v-if="tvShow.episode_run_time && tvShow.episode_run_time.length">
                <span class="meta-label">Episode Runtime</span>
                <span class="meta-value">{{ tvShow.episode_run_time[0] }} min</span>
              </div>
              <div class="meta-separator">•</div>
              <div class="meta-item">
                <span class="meta-label">Status</span>
                <span class="meta-value">{{ tvShow.status }}</span>
              </div>
              <div class="meta-separator">•</div>
              <div class="rating-container">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                  <path d="M12 2L15.09 8.26L22 9.27L17 14.14L18.18 21.02L12 17.77L5.82 21.02L7 14.14L2 9.27L8.91 8.26L12 2Z" fill="currentColor"/>
                </svg>
                <span class="rating-value">{{ tvShow.vote_average.toFixed(1) }}</span>
                <span class="rating-max">/10</span>
              </div>
            </div>
          </div>

          <!-- Genres -->
          <div class="genres-section" v-if="tvShow.genres && tvShow.genres.length">
            <h3 class="section-title">Genres</h3>
            <div class="genres-list">
              <span v-for="genre in tvShow.genres" :key="genre" class="genre-tag">
                {{ genre }}
              </span>
            </div>
          </div>

          <!-- Rating Section -->
          <div class="rating-section">
            <h3 class="section-title">Your Rating</h3>
            <StarRating
              v-model="currentRating"
              @change="handleRatingChange"
              @clear="handleRatingClear"
              :show-rating-text="true"
              :show-clear-button="true"
              size="large"
            />
          </div>

          <!-- Overview -->
          <div class="overview-section">
            <h3 class="section-title">Overview</h3>
            <p class="overview-text">{{ tvShow.overview }}</p>
          </div>

          <!-- Additional details -->
          <div class="details-grid">
            <div class="detail-item" v-if="tvShow.first_air_date">
              <span class="detail-label">First Air Date</span>
              <span class="detail-value">{{ formatDate(tvShow.first_air_date) }}</span>
            </div>
            <div class="detail-item" v-if="tvShow.number_of_seasons">
              <span class="detail-label">Seasons</span>
              <span class="detail-value">{{ tvShow.number_of_seasons }}</span>
            </div>
            <div class="detail-item" v-if="tvShow.number_of_episodes">
              <span class="detail-label">Episodes</span>
              <span class="detail-value">{{ tvShow.number_of_episodes }}</span>
            </div>
            <div class="detail-item" v-if="tvShow.status">
              <span class="detail-label">Status</span>
              <span class="detail-value">{{ tvShow.status }}</span>
            </div>
            <div class="detail-item" v-if="tvShow.episode_run_time && tvShow.episode_run_time.length">
              <span class="detail-label">Avg Episode Length</span>
              <span class="detail-value">{{ formatRuntime(tvShow.episode_run_time[0]) }}</span>
            </div>
          </div>

          <!-- Notes Section -->
          <NotesSection
            v-if="tvShow"
            :content-id="tvShow.id"
            content-type="tv-show"
            :content-title="tvShow.name"
          />
        </div>
      </div>
    </div>

    <!-- No data state -->
    <div v-if="!tvShow && !loading" class="no-data-container">
      <div class="no-data-content">
        <svg width="64" height="64" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
          <path d="M12 9V13M12 17H12.01M21 12C21 16.9706 16.9706 21 12 21C7.02944 21 3 16.9706 3 12C3 7.02944 7.02944 3 12 3C16.9706 3 21 7.02944 21 12Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
        <h3>TV Show Not Found</h3>
        <p>The TV show data could not be loaded. Please try again.</p>
        <button @click="goBack" class="btn-primary">Go Back</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import NotesSection from '../components/NotesSection.vue';
import StarRating from '../components/StarRating.vue';
import { useToast } from '../composables/useToast';
import { getTvShowDetails } from '../services/tmdbService';
import { useRatingStore } from '../stores/ratingStore';
import { useWatchlistStore } from '../stores/watchlistStore';
import { invoke } from '@tauri-apps/api/core';
import { ref, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';

const tvShow = ref<any>(null);
const loading = ref(true);
const currentRating = ref(0);
const route = useRoute();
const router = useRouter();
const { isTvShowInWatchlist, addTvShowToWatchlist, removeTvShowFromWatchlist } = useWatchlistStore();
const { getTvShowRating, rateTvShow, removeTvShowRating } = useRatingStore();
const { success, error } = useToast();

const fetchTvShow = async (showId: number) => {
  try {
    loading.value = true;
    tvShow.value = await getTvShowDetails(invoke, showId);

    // Load existing rating if any
    const existingRating = getTvShowRating.value(showId);
    currentRating.value = existingRating;
  } catch (error) {
    console.error('Failed to load TV show details:', error);
  } finally {
    loading.value = false;
  }
};

const goBack = () => {
  // Get navigation context from localStorage
  const contextStr = localStorage.getItem('tvShowNavigationContext');

  if (contextStr) {
    try {
      const context = JSON.parse(contextStr);

      if (context.from === 'watchlist') {
        router.push({
          path: '/watchlist',
          query: { tab: 'tvShows' }
        });
      } else if (context.from === 'popular') {
        router.push({
          path: '/popular',
          query: { tab: 'tvShows' }
        });
      } else {
        router.go(-1);
      }

      // Clean up the context after use
      localStorage.removeItem('tvShowNavigationContext');
    } catch (error) {
      console.error('Error parsing navigation context:', error);
      router.go(-1);
    }
  } else {
    router.go(-1);
  }
};

const toggleWatchlist = async () => {
  if (!tvShow.value) return;

  try {
    if (isTvShowInWatchlist(tvShow.value.id)) {
      await removeTvShowFromWatchlist(tvShow.value);
      success('Removed from Watchlist', `${tvShow.value.name} has been removed from your watchlist`);
    } else {
      await addTvShowToWatchlist(tvShow.value);
      success('Added to Watchlist', `${tvShow.value.name} has been added to your watchlist`);
    }
  } catch (err) {
    console.error('Failed to update watchlist:', err);
    error('Watchlist Error', 'Failed to update your watchlist. Please try again.');
  }
};

const shareShow = () => {
  if (!tvShow.value) return;

  if (navigator.share) {
    navigator.share({
      title: tvShow.value.name,
      text: tvShow.value.overview,
      url: window.location.href,
    });
  } else {
    // Fallback: copy to clipboard
    navigator.clipboard.writeText(window.location.href);
    success('Link Copied', 'Show link has been copied to clipboard');
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
  if (hours > 0) {
    return `${hours}h ${mins}m`;
  }
  return `${mins}m`;
};

const handleRatingChange = async (rating: number) => {
  if (!tvShow.value) return;

  try {
    await rateTvShow(tvShow.value, rating);
    currentRating.value = rating;
    success('TV Show Rated', `You rated ${tvShow.value.name} ${rating}/5 stars`);
  } catch (err) {
    console.error('Failed to rate TV show:', err);
    error('Rating Error', 'Failed to save your rating. Please try again.');
    // Revert the rating on error
    currentRating.value = getTvShowRating.value(tvShow.value.id);
  }
};

const handleRatingClear = async () => {
  if (!tvShow.value) return;

  try {
    await removeTvShowRating(tvShow.value.id);
    currentRating.value = 0;
    success('Rating Removed', `Removed your rating for ${tvShow.value.name}`);
  } catch (err) {
    console.error('Failed to remove TV show rating:', err);
    error('Rating Error', 'Failed to remove your rating. Please try again.');
  }
};

onMounted(() => {
  const showId = route.params.id;
  if (showId) {
    fetchTvShow(Number(showId));
  } else {
    console.error('TV Show ID is missing. Ensure it is passed via router state.');
    loading.value = false;
  }
});
</script>

<style scoped>
.tvshow-details-page {
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

/* TV Show Details */
.tvshow-details-container {
  position: relative;
  min-height: 100vh;
}

.tvshow-background {
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

.tvshow-nav {
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

.tvshow-content {
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

.tvshow-poster {
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

/* TV Show Info Section */
.tvshow-info-section {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-2xl);
}

.tvshow-header {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-md);
}

.tvshow-title {
  font-size: 3rem;
  font-weight: 700;
  color: var(--color-text-primary);
  line-height: 1.2;
  margin: 0;
}

.tvshow-meta {
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

.btn-primary {
  background: var(--color-accent-primary);
  color: white;
  padding: var(--spacing-md) var(--spacing-lg);
  border: none;
  border-radius: var(--radius-medium);
  font-weight: 500;
  cursor: pointer;
  transition: all var(--transition-fast);
  min-height: auto;
}

.btn-primary:hover {
  background: var(--color-accent-primary-hover);
  transform: translateY(-1px);
}

/* Responsive Design */
@media (max-width: 1024px) {
  .tvshow-content {
    grid-template-columns: 250px 1fr;
    gap: var(--spacing-xl);
  }

  .tvshow-title {
    font-size: 2.5rem;
  }
}

@media (max-width: 768px) {
  .tvshow-content {
    grid-template-columns: 1fr;
    gap: var(--spacing-lg);
    padding: var(--spacing-lg);
  }

  .poster-container {
    position: static;
    max-width: 300px;
    margin: 0 auto;
  }

  .tvshow-title {
    font-size: 2rem;
    text-align: center;
  }

  .tvshow-meta {
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
  .tvshow-nav {
    padding: var(--spacing-md);
  }

  .tvshow-content {
    padding: var(--spacing-md);
  }

  .tvshow-title {
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
