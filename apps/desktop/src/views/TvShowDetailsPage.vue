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

        </div>
      </div>

      <!-- Notes Section -->
      <div v-if="tvShow" class="notes-section">
        <div class="section-header">
          <h3 class="section-title">
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M14 2H6C4.89543 2 4 2.89543 4 4V20C4 21.1046 4.89543 22 6 22H18C19.1046 22 20 21.1046 20 20V8L14 2Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              <polyline points="14,2 14,8 20,8" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              <line x1="16" y1="13" x2="8" y2="13" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              <line x1="16" y1="17" x2="8" y2="17" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              <polyline points="10,9 9,9 8,9" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
            My Notes
          </h3>

          <div class="section-actions" v-if="!isEditingNotes">
            <button
              @click="startEditingNotes"
              class="action-btn edit-btn"
              :title="hasNotes ? 'Edit notes' : 'Add notes'"
            >
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M11 4H4C2.89543 4 2 4.89543 2 6V20C2 21.1046 2.89543 22 4 22H18C19.1046 22 20 21.1046 20 20V13" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                <path d="M18.5 2.50023C18.8978 2.1024 19.4374 1.87891 20 1.87891C20.5626 1.87891 21.1022 2.1024 21.5 2.50023C21.8978 2.89805 22.1213 3.43762 22.1213 4.00023C22.1213 4.56284 21.8978 5.1024 21.5 5.50023L12 15.0002L8 16.0002L9 12.0002L18.5 2.50023Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
              {{ hasNotes ? 'Edit' : 'Add Notes' }}
            </button>
          </div>
        </div>

        <!-- Loading State -->
        <div v-if="isNotesLoading" class="loading-state">
          <div class="loading-spinner"></div>
          <p>Loading notes...</p>
        </div>

        <!-- Error State -->
        <div v-else-if="notesError" class="error-state">
          <svg width="48" height="48" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="2"/>
            <line x1="15" y1="9" x2="9" y2="15" stroke="currentColor" stroke-width="2"/>
            <line x1="9" y1="9" x2="15" y2="15" stroke="currentColor" stroke-width="2"/>
          </svg>
          <h4>Failed to load notes</h4>
          <p>{{ notesError }}</p>
          <button @click="loadNotes" class="retry-btn">Try Again</button>
        </div>

        <!-- Edit Mode -->
        <div v-else-if="isEditingNotes" class="edit-mode">
          <MarkdownEditor
            v-model="editingNotes"
            :title="`Notes for ${tvShow.name}`"
            :placeholder="notesPlaceholder"
            :max-length="5000"
            :show-actions="true"
            :save-button-text="hasNotes ? 'Update Notes' : 'Save Notes'"
            @save="saveNotes"
            @cancel="cancelEditingNotes"
          />
        </div>

        <!-- Display Mode -->
        <div v-else class="display-mode">
          <!-- Empty State -->
          <div v-if="!hasNotes" class="empty-notes">
            <svg width="64" height="64" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M14 2H6C4.89543 2 4 2.89543 4 4V20C4 21.1046 4.89543 22 6 22H18C19.1046 22 20 21.1046 20 20V8L14 2Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              <polyline points="14,2 14,8 20,8" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
            <h4>No notes yet</h4>
            <p>Add your personal notes, thoughts, or reminders about {{ tvShow.name }}.</p>
            <button @click="startEditingNotes" class="add-notes-btn">
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <line x1="12" y1="5" x2="12" y2="19" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                <line x1="5" y1="12" x2="19" y2="12" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
              Add Notes
            </button>
          </div>

          <!-- Rendered Notes -->
          <div v-else class="notes-display">
            <div class="notes-content" v-html="renderedNotes"></div>
            <div class="notes-meta">
              <span class="last-updated">Last updated: {{ formatNoteDate(notesLastUpdated) }}</span>
              <div class="notes-actions">
                <button @click="startEditingNotes" class="action-btn small edit-btn">
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path d="M11 4H4C2.89543 4 2 4.89543 2 6V20C2 21.1046 2.89543 22 4 22H18C19.1046 22 20 21.1046 20 20V13" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                    <path d="M18.5 2.50023C18.8978 2.1024 19.4374 1.87891 20 1.87891C20.5626 1.87891 21.1022 2.1024 21.5 2.50023C21.8978 2.89805 22.1213 3.43762 22.1213 4.00023C22.1213 4.56284 21.8978 5.1024 21.5 5.50023L12 15.0002L8 16.0002L9 12.0002L18.5 2.50023Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                  </svg>
                  Edit
                </button>
                <button @click="clearNotes" class="action-btn small delete-btn">
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <polyline points="3,6 5,6 21,6" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                    <path d="M19,6V20C19,21.1046 18.1046,22 17,22H7C5.89543,22 5,21.1046 5,20V6M8,6V4C8,2.89543 8.89543,2 10,2H14C15.1046,2 16,2.89543 16,4V6" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                  </svg>
                  Clear
                </button>
              </div>
            </div>
          </div>
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
import MarkdownEditor from '../components/MarkdownEditor.vue';
import StarRating from '../components/StarRating.vue';
import { useToast } from '../composables/useToast';
import { getTvShowDetails } from '../services/tmdbService';
import { useRatingStore } from '../stores/ratingStore';
import { useWatchlistStore } from '../stores/watchlistStore';
import { invoke } from '@tauri-apps/api/core';
import { ref, onMounted, computed, watch } from 'vue';
import { useRoute, useRouter } from 'vue-router';

const tvShow = ref<any>(null);
const loading = ref(true);
const currentRating = ref(0);

// Notes reactive state
const notes = ref('');
const editingNotes = ref('');
const renderedNotes = ref('');
const isEditingNotes = ref(false);
const isNotesLoading = ref(false);
const isSavingNotes = ref(false);
const notesError = ref('');
const notesLastUpdated = ref<Date | null>(null);

const route = useRoute();
const router = useRouter();
const { isTvShowInWatchlist, addTvShowToWatchlist, removeTvShowFromWatchlist } = useWatchlistStore();
const { getTvShowRating, rateTvShow, removeTvShowRating } = useRatingStore();
const { success, error } = useToast();

// Notes computed properties
const hasNotes = computed(() => notes.value.trim().length > 0);
const notesPlaceholder = computed(() => {
  return `Write your thoughts about this TV show...\n\n**What did you think?**\n- Great story\n- Amazing performances\n- Would watch again\n\n*Use markdown for formatting!*`
});

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

// Notes methods
const loadNotes = async () => {
  if (isNotesLoading.value || !tvShow.value) return;

  isNotesLoading.value = true;
  notesError.value = '';

  try {
    const result = await invoke('get_tv_show_notes', { tvShowId: tvShow.value.id }) as string | null;
    notes.value = result || '';

    if (notes.value) {
      await renderNotes();
      notesLastUpdated.value = new Date(); // In a real app, this would come from the backend
    }
  } catch (err) {
    console.error('Failed to load notes:', err);
    notesError.value = 'Failed to load notes. Please try again.';
  } finally {
    isNotesLoading.value = false;
  }
};

const renderNotes = async () => {
  if (!notes.value.trim()) {
    renderedNotes.value = '';
    return;
  }

  try {
    const html = await invoke('render_markdown_to_html', {
      markdown: notes.value
    }) as string;
    renderedNotes.value = html;
  } catch (err) {
    console.error('Failed to render notes:', err);
    renderedNotes.value = `<p style="color: var(--color-error);">Failed to render notes</p>`;
  }
};

const startEditingNotes = () => {
  editingNotes.value = notes.value;
  isEditingNotes.value = true;
};

const cancelEditingNotes = () => {
  editingNotes.value = '';
  isEditingNotes.value = false;
};

const saveNotes = async (newNotes: string) => {
  if (isSavingNotes.value || !tvShow.value) return;

  isSavingNotes.value = true;

  try {
    await invoke('update_tv_show_notes', {
      tvShowId: tvShow.value.id,
      notes: newNotes
    });

    notes.value = newNotes;
    notesLastUpdated.value = new Date();
    isEditingNotes.value = false;

    if (newNotes.trim()) {
      await renderNotes();
      success('Notes Saved', 'Your notes have been saved successfully');
    } else {
      renderedNotes.value = '';
      success('Notes Cleared', 'Your notes have been cleared');
    }
  } catch (err) {
    console.error('Failed to save notes:', err);
    error('Save Failed', 'Failed to save your notes. Please try again.');
  } finally {
    isSavingNotes.value = false;
  }
};

const clearNotes = async () => {
  if (!confirm('Are you sure you want to clear all notes? This action cannot be undone.')) {
    return;
  }

  await saveNotes('');
};

const formatNoteDate = (date: Date | null) => {
  if (!date) return 'Unknown';

  const now = new Date();
  const diffMs = now.getTime() - date.getTime();
  const diffMins = Math.floor(diffMs / (1000 * 60));
  const diffHours = Math.floor(diffMs / (1000 * 60 * 60));
  const diffDays = Math.floor(diffMs / (1000 * 60 * 60 * 24));

  if (diffMins < 1) return 'Just now';
  if (diffMins < 60) return `${diffMins} minute${diffMins === 1 ? '' : 's'} ago`;
  if (diffHours < 24) return `${diffHours} hour${diffHours === 1 ? '' : 's'} ago`;
  if (diffDays < 7) return `${diffDays} day${diffDays === 1 ? '' : 's'} ago`;

  return date.toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric'
  });
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

// Watch for tvShow changes to load notes
watch(tvShow, (newTvShow) => {
  if (newTvShow) {
    loadNotes();
  }
}, { immediate: false });
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

/* Notes Section Styles */
.notes-section {
  background: var(--color-surface);
  border-radius: var(--radius-large);
  border: 1px solid var(--color-border);
  padding: var(--spacing-xl);
  margin: var(--spacing-xl) var(--spacing-xl) 0;
  max-width: 1200px;
  margin-left: auto;
  margin-right: auto;
}

.section-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: var(--spacing-lg);
}

.section-title {
  font-size: 1.25rem;
  font-weight: 600;
  color: var(--color-text-primary);
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  margin: 0;
}

.section-actions {
  display: flex;
  gap: var(--spacing-sm);
}

.action-btn {
  background: var(--color-background);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  padding: var(--spacing-sm) var(--spacing-md);
  font-size: 0.875rem;
  font-weight: 500;
  color: var(--color-text-secondary);
  cursor: pointer;
  transition: all 0.2s ease;
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
}

.action-btn:hover {
  background: var(--color-surface);
  color: var(--color-text-primary);
}

.action-btn.small {
  padding: var(--spacing-xs) var(--spacing-sm);
  font-size: 0.8125rem;
}

.action-btn.edit-btn {
  color: var(--color-primary);
}

.action-btn.delete-btn {
  color: var(--color-error);
}

.action-btn.delete-btn:hover {
  background: var(--color-error-background);
}

.loading-state, .error-state, .empty-notes {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: var(--spacing-xl);
  text-align: center;
  color: var(--color-text-secondary);
}

.loading-spinner {
  width: 32px;
  height: 32px;
  border: 3px solid var(--color-border);
  border-top: 3px solid var(--color-primary);
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin-bottom: var(--spacing-md);
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

.error-state svg, .empty-notes svg {
  margin-bottom: var(--spacing-md);
  color: var(--color-text-tertiary);
}

.error-state h4, .empty-notes h4 {
  font-size: 1.125rem;
  color: var(--color-text-primary);
  margin: 0 0 var(--spacing-sm) 0;
}

.error-state p, .empty-notes p {
  margin: 0 0 var(--spacing-lg) 0;
}

.retry-btn, .add-notes-btn {
  background: var(--color-primary);
  color: var(--color-primary-text);
  border: none;
  border-radius: var(--radius-md);
  padding: var(--spacing-sm) var(--spacing-lg);
  font-size: 0.875rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
}

.retry-btn:hover, .add-notes-btn:hover {
  background: var(--color-primary-hover);
}

.edit-mode {
  margin-top: var(--spacing-md);
}

.display-mode {
  margin-top: var(--spacing-md);
}

.notes-display {
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
}

.notes-content {
  padding: var(--spacing-lg);
  background: var(--color-background);
}

.notes-content :deep(h1), .notes-content :deep(h2), .notes-content :deep(h3), .notes-content :deep(h4), .notes-content :deep(h5), .notes-content :deep(h6) {
  color: var(--color-text-primary);
  font-weight: 600;
  margin: 0 0 var(--spacing-md) 0;
  line-height: 1.4;
}

.notes-content :deep(h1) {
  font-size: 1.5rem;
  border-bottom: 2px solid var(--color-border);
  padding-bottom: var(--spacing-sm);
}

.notes-content :deep(h2) {
  font-size: 1.25rem;
  border-bottom: 1px solid var(--color-border);
  padding-bottom: var(--spacing-xs);
}

.notes-content :deep(h3) {
  font-size: 1.125rem;
}

.notes-content :deep(p) {
  margin: 0 0 var(--spacing-md) 0;
}

.notes-content :deep(ul), .notes-content :deep(ol) {
  margin: 0 0 var(--spacing-md) 0;
  padding-left: var(--spacing-lg);
}

.notes-content :deep(li) {
  margin-bottom: var(--spacing-xs);
}

.notes-content :deep(strong) {
  font-weight: 600;
  color: var(--color-text-primary);
}

.notes-content :deep(em) {
  font-style: italic;
  color: var(--color-text-secondary);
}

.notes-content :deep(a) {
  color: var(--color-primary);
  text-decoration: none;
  border-bottom: 1px solid transparent;
  transition: border-color 0.2s ease;
}

.notes-content :deep(a:hover) {
  border-bottom-color: var(--color-primary);
}

.notes-content :deep(code) {
  background: var(--color-surface);
  color: var(--color-text-primary);
  padding: 0.125rem 0.25rem;
  border-radius: var(--radius-sm);
  font-family: 'SF Mono', Monaco, 'Cascadia Code', 'Roboto Mono', Consolas, 'Courier New', monospace;
  font-size: 0.875em;
}

.notes-content :deep(blockquote) {
  border-left: 4px solid var(--color-primary);
  margin: var(--spacing-md) 0;
  padding: var(--spacing-sm) var(--spacing-md);
  background: var(--color-surface);
  color: var(--color-text-secondary);
}

.notes-meta {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-md) var(--spacing-lg);
  background: var(--color-surface);
  border-top: 1px solid var(--color-border);
  font-size: 0.875rem;
  color: var(--color-text-tertiary);
}

.notes-actions {
  display: flex;
  gap: var(--spacing-sm);
}

@media (max-width: 768px) {
  .notes-section {
    margin: var(--spacing-lg) var(--spacing-md) 0;
  }
  
  .section-header {
    flex-direction: column;
    align-items: flex-start;
    gap: var(--spacing-md);
  }

  .notes-meta {
    flex-direction: column;
    align-items: flex-start;
    gap: var(--spacing-sm);
  }

  .notes-actions {
    width: 100%;
    justify-content: flex-end;
  }

  .loading-state, .error-state, .empty-notes {
    padding: var(--spacing-lg);
  }
}
</style>
