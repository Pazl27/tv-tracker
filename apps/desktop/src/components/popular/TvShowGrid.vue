<template>
  <div class="tvshow-grid-container">
    <!-- Skeleton loaders -->
    <div v-if="loading" class="tvshow-grid">
      <div v-for="n in 20" :key="n" class="skeleton-card">
        <div class="skeleton-poster animate-shimmer"></div>
        <div class="skeleton-content">
          <div class="skeleton-title animate-shimmer"></div>
          <div class="skeleton-subtitle animate-shimmer"></div>
        </div>
      </div>
    </div>

    <!-- TV Show Cards -->
    <div v-else class="tvshow-grid">
      <div
        v-for="show in tvShows"
        :key="show.id"
        class="tvshow-card"
        @click="goToShowDetails(show)"
      >
        <div class="tvshow-poster-container">
          <LazyImage
            :src="show.poster_url"
            :alt="show.name"
            aspect-ratio="2/3"
            quality="medium"
            :show-spinner="true"
            root-margin="100px"
            class="tvshow-poster"
            @load="onImageLoad"
            @error="onImageError"
            @visible="onImageVisible"
          />
          <div class="tvshow-overlay">
            <button
              class="action-button add-button"
              @click.stop="toggleWatchlist(show)"
              :title="isTvShowInWatchlist(show.id) ? 'Remove from Watchlist' : 'Add to Watchlist'"
              :class="{ 'in-watchlist': isTvShowInWatchlist(show.id) }"
            >
              <svg v-if="!isTvShowInWatchlist(show.id)" width="20" height="20" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M12 5V19M5 12H19" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
              <svg v-else width="20" height="20" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M9 12L11 14L15 10M21 12C21 16.9706 16.9706 21 12 21C7.02944 21 3 16.9706 3 12C3 7.02944 7.02944 3 12 3C16.9706 3 21 7.02944 21 12Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
            </button>

          </div>
          <div class="rating-badge" v-if="show.vote_average">
            <svg width="12" height="12" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M12 2L15.09 8.26L22 9.27L17 14.14L18.18 21.02L12 17.77L5.82 21.02L7 14.14L2 9.27L8.91 8.26L12 2Z" fill="currentColor"/>
            </svg>
            <span>{{ show.vote_average.toFixed(1) }}</span>
          </div>
          <div class="tv-badge">
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M7 21H17C18.1046 21 19 20.1046 19 19V5C19 3.89543 18.1046 3 17 3H7C5.89543 3 5 3.89543 5 5V19C5 20.1046 5.89543 21 7 21ZM9 9H15V15H9V9Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
            <span>TV</span>
          </div>
        </div>

        <div class="tvshow-content">
          <h3 class="tvshow-title">{{ show.name }}</h3>
          <p class="tvshow-year" v-if="show.first_air_date">
            {{ new Date(show.first_air_date).getFullYear() }}
          </p>
        </div>
      </div>
    </div>

    <!-- Empty state -->
    <div v-if="!loading && tvShows.length === 0" class="empty-state">
      <svg width="64" height="64" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path d="M7 21H17C18.1046 21 19 20.1046 19 19V5C19 3.89543 18.1046 3 17 3H7C5.89543 3 5 3.89543 5 5V19C5 20.1046 5.89543 21 7 21ZM9 9H15V15H9V9Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>
      <h3>No TV shows found</h3>
      <p>Try adjusting your search criteria</p>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { useToast } from '../../composables/useToast';
import { fetchTvShows } from '../../services/tmdbService';
import { useWatchlistStore } from '../../stores/watchlistStore';
import { invoke } from "@tauri-apps/api/core";
import { ref, onMounted, watch, defineProps } from 'vue';
import { useRouter } from 'vue-router';

const props = defineProps<{ searchedTvShows: any[] }>();

const tvShows = ref<any[]>(props.searchedTvShows || []);
const loading = ref(true);

const router = useRouter();
const { isTvShowInWatchlist, addTvShowToWatchlist, removeTvShowFromWatchlist } = useWatchlistStore();
const { success, error } = useToast();

// Image event handlers
const onImageLoad = (_event: Event) => {
  // Optional: handle successful image loads
};

const onImageError = (_event: Event) => {
  // Optional: handle image load errors
};

const onImageVisible = () => {
  // Optional: handle when image becomes visible
};

// Fetch trending TV shows
const loadTvShows = async () => {
  try {
    tvShows.value = await fetchTvShows(invoke);
  } catch (error) {
    console.error('Failed to load TV shows:', error);
  } finally {
    loading.value = false;
  }
};

const toggleWatchlist = async (show: any) => {
  try {
    if (isTvShowInWatchlist(show.id)) {
      await removeTvShowFromWatchlist(show);
      success('Removed from Watchlist', `${show.name} has been removed from your watchlist`);
    } else {
      await addTvShowToWatchlist(show);
      success('Added to Watchlist', `${show.name} has been added to your watchlist`);
    }
  } catch (err) {
    console.error('Failed to update watchlist:', err);
    error('Watchlist Error', 'Failed to update your watchlist. Please try again.');
  }
};

const goToShowDetails = (show: any) => {
  localStorage.setItem('selectedTvShow', JSON.stringify(show));
  localStorage.setItem('tvShowNavigationContext', JSON.stringify({
    from: 'popular',
    tab: 'tvShows'
  }));
  router.push({
    name: 'TvShowDetails',
    params: { id: show.id }
  });
};

// Fetch initial TV shows when component mounts
onMounted(() => {
  if (tvShows.value.length === 0) {
    loadTvShows();
  } else {
    loading.value = false;
  }
});

watch(() => props.searchedTvShows, (newTvShows) => {
  if (newTvShows && newTvShows.length > 0) {
    tvShows.value = newTvShows;
    loading.value = false;
  }
});
</script>

<style scoped>
.tvshow-grid-container {
  padding: var(--spacing-lg);
  max-width: 1400px;
  margin: 0 auto;
}

.tvshow-grid {
  display: grid;
  gap: var(--spacing-lg);
  grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
}

/* TV Show Card Styles */
.tvshow-card {
  background: var(--color-card-background);
  border: 1px solid var(--color-card-border);
  border-radius: var(--radius-large);
  overflow: hidden;
  cursor: pointer;
  transition: all var(--transition-medium);
  box-shadow: var(--shadow-small);
  position: relative;
}

.tvshow-card:hover {
  transform: translateY(-4px);
  box-shadow: var(--shadow-large);
  border-color: var(--color-accent-primary);
}

.tvshow-card:hover .tvshow-content {
  background: linear-gradient(135deg, var(--color-card-background) 0%, var(--color-surface) 100%);
}

.tvshow-card:active {
  transform: translateY(-2px);
  transition: transform var(--transition-fast);
}

.tvshow-poster-container {
  position: relative;
  overflow: hidden;
  aspect-ratio: 2/3;
}

.tvshow-poster {
  transition: transform var(--transition-medium);
}

.tvshow-card:hover .tvshow-poster {
  transform: scale(1.05);
}

.tvshow-overlay {
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

.tvshow-card:hover .tvshow-overlay {
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

.tv-badge {
  position: absolute;
  top: var(--spacing-sm);
  left: var(--spacing-sm);
  background: rgba(99, 102, 241, 0.9);
  backdrop-filter: blur(8px);
  color: white;
  padding: var(--spacing-xs) var(--spacing-sm);
  border-radius: var(--radius-medium);
  font-size: 0.75rem;
  font-weight: 600;
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  border: 1px solid rgba(99, 102, 241, 0.3);
}

.tvshow-content {
  padding: var(--spacing-md);
  transition: background var(--transition-medium);
  position: relative;
}

.tvshow-title {
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

.tvshow-card:hover .tvshow-title {
  color: var(--color-accent-primary);
}

.tvshow-year {
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
  .tvshow-grid {
    grid-template-columns: repeat(8, 1fr);
  }
}

@media (min-width: 1200px) and (max-width: 1599px) {
  .tvshow-grid {
    grid-template-columns: repeat(6, 1fr);
  }
}

@media (min-width: 768px) and (max-width: 1199px) {
  .tvshow-grid {
    grid-template-columns: repeat(4, 1fr);
  }
}

@media (min-width: 480px) and (max-width: 767px) {
  .tvshow-grid {
    grid-template-columns: repeat(3, 1fr);
    gap: var(--spacing-md);
  }

  .tvshow-grid-container {
    padding: var(--spacing-md);
  }
}

@media (max-width: 479px) {
  .tvshow-grid {
    grid-template-columns: repeat(2, 1fr);
    gap: var(--spacing-sm);
  }

  .tvshow-grid-container {
    padding: var(--spacing-sm);
  }

  .tvshow-content {
    padding: var(--spacing-sm);
  }

  .tvshow-title {
    font-size: 0.875rem;
  }

  .tvshow-year {
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
