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

      </div>

      <!-- Notes Section -->
      <div v-if="movie" class="notes-section">
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
            :title="`Notes for ${movie.title}`"
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
            <p>Add your personal notes, thoughts, or reminders about {{ movie.title }}.</p>
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
import MarkdownEditor from '../components/MarkdownEditor.vue';
import RatingPopup from '../components/RatingPopup.vue';
import StarRating from '../components/StarRating.vue';
import { useToast } from '../composables/useToast';
import { getMovieDetails } from '../services/tmdbService';
import { useRatingStore } from '../stores/ratingStore';
import { useWatchlistStore } from '../stores/watchlistStore';
import { invoke } from '@tauri-apps/api/core';
import { ref, onMounted, computed, watch } from 'vue';
import { useRoute, useRouter } from 'vue-router';

const movie = ref<any>(null);
const loading = ref(true);
const currentRating = ref(0);
const showRatingPopup = ref(false);
const existingWatchedAt = ref('');

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
const { isMovieInWatchlist, addMovieToWatchlist, removeMovieFromWatchlist } = useWatchlistStore();
const { getMovieRating, rateMovie, removeMovieRating } = useRatingStore();
const { success, error } = useToast();

// Notes computed properties
const hasNotes = computed(() => notes.value.trim().length > 0);
const notesPlaceholder = computed(() => {
  return `Write your thoughts about this movie...\n\n**What did you think?**\n- Great story\n- Amazing performances\n- Would watch again\n\n*Use markdown for formatting!*`
});

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

// Notes methods
const loadNotes = async () => {
  if (isNotesLoading.value || !movie.value) return;

  isNotesLoading.value = true;
  notesError.value = '';

  try {
    const result = await invoke('get_movie_notes', { movieId: movie.value.id }) as string | null;
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
  if (isSavingNotes.value || !movie.value) return;

  isSavingNotes.value = true;

  try {
    await invoke('update_movie_notes', {
      movieId: movie.value.id,
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

// Watch for movie changes to load notes
watch(movie, (newMovie) => {
  if (newMovie) {
    loadNotes();
  }
}, { immediate: false });
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
