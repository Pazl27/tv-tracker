<template>
  <div class="watched-page">
    <!-- Header -->
    <div class="page-header">
      <div class="header-top">
        <button @click="goBack" class="back-button">
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M19 12H5M5 12L12 19M5 12L12 5" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
          <span>Back</span>
        </button>
        <div class="header-text">
          <h1 class="page-title">Watched Content</h1>
          <p class="page-subtitle">Your rated movies and TV shows</p>
        </div>
      </div>

      <!-- Search Bar -->
      <div class="search-container">
        <div class="search-bar">
          <svg class="search-icon" width="20" height="20" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M21 21L16.514 16.506L21 21ZM19 10.5C19 15.194 15.194 19 10.5 19C5.806 19 2 15.194 2 10.5C2 5.806 5.806 2 10.5 2C15.194 2 19 5.806 19 10.5Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
          <input
            ref="searchInput"
            v-model="searchQuery"
            type="text"
            placeholder="Search your watched movies and TV shows..."
            @input="handleSearch"
            @keydown.escape="clearSearch"
            @keydown.enter="focusFirstResult"
            class="search-input"
          />
          <button
            v-if="searchQuery"
            @click="clearSearch"
            class="clear-button"
            type="button"
          >
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M18 6L6 18M6 6L18 18" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
          </button>
        </div>
      </div>

      <!-- Quick Filters - Always visible -->
      <div class="quick-filters">
        <div class="filter-group">
          <span class="filter-label">Filter by Rating:</span>
          <button
            v-for="rating in [5, 4, 3, 2, 1]"
            :key="rating"
            @click="toggleRatingFilter(rating)"
            :class="['filter-btn', { active: ratingFilters.includes(rating) }]"
          >
            {{ rating }}+ ⭐
          </button>
          <button
            v-if="ratingFilters.length > 0"
            @click="clearRatingFilters"
            class="filter-btn clear"
          >
            Clear Filters
          </button>
        </div>
      </div>
    </div>

    <!-- Search Results Summary -->
    <div v-if="searchQuery" class="search-results">
      <p class="search-results-text">
        <span v-if="activeTab === 'movies'">
          {{ filteredMovies.length }} of {{ ratedMovies.length }} movies
        </span>
        <span v-else>
          {{ filteredTvShows.length }} of {{ ratedTvShows.length }} TV shows
        </span>
        match "{{ searchQuery }}"
      </p>
    </div>

    <!-- Tabs -->
    <div class="content-tabs">
      <button
        class="tab-button"
        :class="{ active: activeTab === 'movies' }"
        @click="activeTab = 'movies'"
      >
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
          <path d="M20 4H4C2.89543 4 2 4.89543 2 6V18C2 19.1046 2.89543 20 4 20H20C21.1046 20 22 19.1046 22 18V6C22 4.89543 21.1046 4 20 4Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          <path d="M8 12L12 8L16 12" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
        Movies
        <span class="tab-count">({{ searchQuery ? filteredMovies.length : ratedMovies.length }})</span>
      </button>
      <button
        class="tab-button"
        :class="{ active: activeTab === 'tvShows' }"
        @click="activeTab = 'tvShows'"
      >
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
          <path d="M7 21H17C18.1046 21 19 20.1046 19 19V5C19 3.89543 18.1046 3 17 3H7C5.89543 3 5 3.89543 5 5V19C5 20.1046 5.89543 21 7 21Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          <path d="M9 9H15V15H9V9Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
        TV Shows
        <span class="tab-count">({{ searchQuery ? filteredTvShows.length : ratedTvShows.length }})</span>
      </button>
    </div>

    <!-- Loading State -->
    <div v-if="isLoading" class="loading-container">
      <div class="loading-spinner"></div>
      <p>Loading your watched content...</p>
    </div>

    <!-- Movies Tab -->
    <div v-else-if="activeTab === 'movies'" class="content-list">
      <div v-if="filteredMovies.length === 0 && !searchQuery" class="empty-state">
        <svg width="64" height="64" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
          <path d="M20 4H4C2.89543 4 2 4.89543 2 6V18C2 19.1046 2.89543 20 4 20H20C21.1046 20 22 19.1046 22 18V6C22 4.89543 21.1046 4 20 4Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          <path d="M8 12L12 8L16 12" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
        <h3>No rated movies yet</h3>
        <p>Start rating movies to see them here</p>
        <router-link to="/popular?tab=movies" class="cta-button">Browse Movies</router-link>
      </div>

      <div v-else-if="filteredMovies.length === 0 && searchQuery" class="empty-state">
        <svg width="64" height="64" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
          <path d="M21 21L16.514 16.506L21 21ZM19 10.5C19 15.194 15.194 19 10.5 19C5.806 19 2 15.194 2 10.5C2 5.806 5.806 2 10.5 2C15.194 2 19 5.806 19 10.5Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
        <h3>No movies found</h3>
        <p>No watched movies match "{{ searchQuery }}"</p>
        <button @click="clearSearch" class="cta-button">Clear Search</button>
      </div>

      <div v-else class="content-items">
        <div
          v-for="movie in filteredMovies"
          :key="movie.id"
          class="content-item"
          @click="navigateToMovie(movie.id)"
        >
          <img
            :src="`https://image.tmdb.org/t/p/w200${movie.poster_path}`"
            :alt="movie.title"
            class="content-poster"
            @error="handleImageError"
          />
          <div class="content-info">
            <h3 class="content-title" v-html="highlightSearchTerm(movie.title)"></h3>
            <div class="content-meta">
              <div class="rating-display">
                <StarRating
                  :model-value="movie.rating"
                  :readonly="true"
                  :show-rating-text="true"
                  :show-clear-button="false"
                  size="small"
                />
              </div>
              <div class="watched-date">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                  <path d="M8 2V6M16 2V6M3 10H21M5 4H19C20.1046 4 21 4.89543 21 6V20C21 21.1046 20.1046 22 19 22H5C3.89543 22 4 21.1046 4 20V6C4 4.89543 3.89543 4 5 4Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                </svg>
                <span>{{ formatWatchedDate(movie.watched_at) }}</span>
              </div>
            </div>
          </div>
          <div class="content-actions">
            <button @click.stop="editRating(movie, 'Movie')" class="action-button edit">
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M11 4H4C3.46957 4 2.96086 4.21071 2.58579 4.58579C2.21071 4.96086 2 5.46957 2 6V20C2 20.5304 2.21071 21.0391 2.58579 21.4142C2.96086 21.7893 3.46957 22 4 22H18C18.5304 22 19.0391 21.7893 19.4142 21.4142C19.7893 21.0391 20 20.5304 20 20V13" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                <path d="M18.5 2.5C18.8978 2.10217 19.4374 1.87868 20 1.87868C20.5626 1.87868 21.1022 2.10217 21.5 2.5C21.8978 2.89783 22.1213 3.43739 22.1213 4C22.1213 4.56261 21.8978 5.10217 21.5 5.5L12 15L8 16L9 12L18.5 2.5Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
            </button>
            <button @click.stop="removeRating(movie.id, 'movie')" class="action-button delete">
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M3 6H5H21M8 6V4C8 3.46957 8.21071 2.96086 8.58579 2.58579C8.96086 2.21071 9.46957 2 10 2H14C14.5304 2 15.0391 2.21071 15.4142 2.58579C15.7893 2.96086 16 3.46957 16 4V6M19 6V20C19 20.5304 18.7893 21.0391 18.4142 21.4142C18.0391 21.7893 17.5304 22 17 22H7C6.46957 22 5.96086 21.7893 5.58579 21.4142C5.21071 21.0391 5 20.5304 5 20V6H19Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- TV Shows Tab -->
    <div v-else class="content-list">
      <div v-if="filteredTvShows.length === 0 && !searchQuery" class="empty-state">
        <svg width="64" height="64" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
          <path d="M7 21H17C18.1046 21 19 20.1046 19 19V5C19 3.89543 18.1046 3 17 3H7C5.89543 3 5 3.89543 5 5V19C5 20.1046 5.89543 21 7 21Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          <path d="M9 9H15V15H9V9Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
        <h3>No rated TV shows yet</h3>
        <p>Start rating TV shows to see them here</p>
        <router-link to="/popular?tab=tvShows" class="cta-button">Browse TV Shows</router-link>
      </div>

      <div v-else-if="filteredTvShows.length === 0 && searchQuery" class="empty-state">
        <svg width="64" height="64" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
          <path d="M21 21L16.514 16.506L21 21ZM19 10.5C19 15.194 15.194 19 10.5 19C5.806 19 2 15.194 2 10.5 2C5.806 5.806 2 10.5 2C15.194 2 19 5.806 19 10.5Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
        <h3>No TV shows found</h3>
        <p>No watched TV shows match "{{ searchQuery }}"</p>
        <button @click="clearSearch" class="cta-button">Clear Search</button>
      </div>

      <div v-else class="content-items">
        <div
          v-for="show in filteredTvShows"
          :key="show.id"
          class="content-item"
          @click="navigateToTvShow(show.id)"
        >
          <img
            :src="`https://image.tmdb.org/t/p/w200${show.poster_path}`"
            :alt="show.name"
            class="content-poster"
            @error="handleImageError"
          />
          <div class="content-info">
            <h3 class="content-title" v-html="highlightSearchTerm(show.name)"></h3>
            <div class="content-meta">
              <div class="rating-display">
                <StarRating
                  :model-value="show.rating"
                  :readonly="true"
                  :show-rating-text="true"
                  :show-clear-button="false"
                  size="small"
                />
              </div>
              <div class="watched-date">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                  <path d="M8 2V6M16 2V6M3 10H21M5 4H19C20.1046 4 21 4.89543 21 6V20C21 21.1046 20.1046 22 19 22H5C3.89543 22 4 21.1046 4 20V6C4 4.89543 3.89543 4 5 4Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                </svg>
                <span>{{ formatWatchedDate(show.watched_at) }}</span>
              </div>
            </div>
          </div>
          <div class="content-actions">
            <button @click.stop="editRating(show, 'TV Show')" class="action-button edit">
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M11 4H4C3.46957 4 2.96086 4.21071 2.58579 4.58579C2.21071 4.96086 2 5.46957 2 6V20C2 20.5304 2.21071 21.0391 2.58579 21.4142C2.96086 21.7893 3.46957 22 4 22H18C18.5304 22 19.0391 21.7893 19.4142 21.4142C19.7893 21.0391 20 20.5304 20 20V13" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                <path d="M18.5 2.5C18.8978 2.10217 19.4374 1.87868 20 1.87868C20.5626 1.87868 21.1022 2.10217 21.5 2.5C21.8978 2.89783 22.1213 3.43739 22.1213 4C22.1213 4.56261 21.8978 5.10217 21.5 5.5L12 15L8 16L9 12L18.5 2.5Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
            </button>
            <button @click.stop="removeRating(show.id, 'tvShow')" class="action-button delete">
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M3 6H5H21M8 6V4C8 3.46957 8.21071 2.96086 8.58579 2.58579C8.96086 2.21071 9.46957 2 10 2H14C14.5304 2 15.0391 2.21071 15.4142 2.58579C15.7893 2.96086 16 3.46957 16 4V6M19 6V20C19 20.5304 18.7893 21.0391 18.4142 21.4142C18.0391 21.7893 17.5304 22 17 22H7C6.46957 22 5.96086 21.7893 5.58579 21.4142C5.21071 21.0391 5 20.5304 5 20V6H19Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- Rating Popup -->
    <RatingPopup
      :is-visible="showRatingPopup"
      :content="selectedContent"
      :content-type="selectedContentType"
      :existing-rating="existingRating"
      :existing-watched-at="existingWatchedAt"
      @close="closeRatingPopup"
      @save="handleRatingSave"
    />
  </div>
</template>

<script setup lang="ts">
import RatingPopup from '../components/RatingPopup.vue'
import StarRating from '../components/StarRating.vue'
import { useToast } from '../composables/useToast'
import { useRatingStore } from '../stores/ratingStore'
import { formatWatchedDate } from '../utils/dateUtils'
import { ref, onMounted, onUnmounted, computed } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()
const {
  ratedMovies,
  ratedTvShows,
  isLoadingMovieRatings,
  isLoadingTvShowRatings,
  loadAllRatedMovies,
  loadAllRatedTvShows,
  rateMovie,
  rateTvShow,
  removeMovieRating,
  removeTvShowRating
} = useRatingStore()
const { success, error } = useToast()

const activeTab = ref('movies')
const showRatingPopup = ref(false)
const selectedContent = ref<any>(null)
const selectedContentType = ref<'Movie' | 'TV Show'>('Movie')
const existingRating = ref(0)
const existingWatchedAt = ref('')
const searchQuery = ref('')
const searchInput = ref<HTMLInputElement | null>(null)
const ratingFilters = ref<number[]>([])

const isLoading = computed(() => {
  return isLoadingMovieRatings.value || isLoadingTvShowRatings.value
})

const filteredMovies = computed(() => {
  let filtered = ratedMovies.value

  // Apply search filter
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    filtered = filtered.filter(movie =>
      movie.title.toLowerCase().includes(query)
    )
  }

  // Apply rating filters
  if (ratingFilters.value.length > 0) {
    filtered = filtered.filter(movie =>
      ratingFilters.value.some(rating => movie.rating >= rating)
    )
  }

  return filtered
})

const filteredTvShows = computed(() => {
  let filtered = ratedTvShows.value

  // Apply search filter
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    filtered = filtered.filter(show =>
      show.name.toLowerCase().includes(query)
    )
  }

  // Apply rating filters
  if (ratingFilters.value.length > 0) {
    filtered = filtered.filter(show =>
      ratingFilters.value.some(rating => show.rating >= rating)
    )
  }

  return filtered
})



const goBack = () => {
  router.go(-1)
}

const handleSearch = () => {
  // Search is reactive via computed properties
  // Auto-switch to the tab with more results if current tab has none
  if (searchQuery.value && activeTab.value === 'movies' && filteredMovies.value.length === 0 && filteredTvShows.value.length > 0) {
    activeTab.value = 'tvShows'
  } else if (searchQuery.value && activeTab.value === 'tvShows' && filteredTvShows.value.length === 0 && filteredMovies.value.length > 0) {
    activeTab.value = 'movies'
  }
}

const clearSearch = () => {
  searchQuery.value = ''
  searchInput.value?.focus()
}

const highlightSearchTerm = (text: string) => {
  if (!searchQuery.value) return text

  const regex = new RegExp(`(${searchQuery.value})`, 'gi')
  return text.replace(regex, '<mark class="search-highlight">$1</mark>')
}

const focusFirstResult = () => {
  const firstItem = document.querySelector('.content-item') as HTMLElement
  if (firstItem) {
    firstItem.click()
  }
}

// Keyboard shortcuts
const handleKeydown = (event: KeyboardEvent) => {
  // Ctrl/Cmd + F to focus search
  if ((event.ctrlKey || event.metaKey) && event.key === 'f') {
    event.preventDefault()
    searchInput.value?.focus()
  }
  // Escape to clear search when not focused on input
  if (event.key === 'Escape' && document.activeElement !== searchInput.value) {
    clearSearch()
  }
}

const toggleRatingFilter = (rating: number) => {
  const index = ratingFilters.value.indexOf(rating)
  if (index > -1) {
    ratingFilters.value.splice(index, 1)
  } else {
    ratingFilters.value.push(rating)
  }
  ratingFilters.value.sort((a, b) => b - a) // Sort descending
}

const clearRatingFilters = () => {
  ratingFilters.value = []
}

const handleImageError = (event: Event) => {
  const target = event.target as HTMLImageElement
  target.src = '/placeholder-poster.png'
}

// formatWatchedDate is now imported from utils/dateUtils

const navigateToMovie = (movieId: number) => {
  // Store navigation context
  localStorage.setItem('movieNavigationContext', JSON.stringify({
    from: 'watched',
    tab: 'movies'
  }))
  router.push(`/movie/${movieId}`)
}

const navigateToTvShow = (showId: number) => {
  // Store navigation context
  localStorage.setItem('tvShowNavigationContext', JSON.stringify({
    from: 'watched',
    tab: 'tvShows'
  }))
  router.push(`/tv/${showId}`)
}

const editRating = (content: any, type: 'Movie' | 'TV Show') => {
  selectedContent.value = content
  selectedContentType.value = type
  existingRating.value = content.rating
  existingWatchedAt.value = content.watched_at
  showRatingPopup.value = true
}

const closeRatingPopup = () => {
  showRatingPopup.value = false
  selectedContent.value = null
  existingRating.value = 0
  existingWatchedAt.value = ''
}

const handleRatingSave = async (data: { rating: number; watchedAt: string }) => {
  try {
    if (selectedContentType.value === 'Movie') {
      await rateMovie(selectedContent.value, data.rating, data.watchedAt)
      success('Rating Updated', `Updated rating for ${selectedContent.value.title}`)
    } else {
      await rateTvShow(selectedContent.value, data.rating, data.watchedAt)
      success('Rating Updated', `Updated rating for ${selectedContent.value.name}`)
    }

    closeRatingPopup()

    // Reload the data
    await loadAllRatedMovies()
    await loadAllRatedTvShows()
  } catch (err) {
    console.error('Failed to update rating:', err)
    error('Update Failed', 'Failed to update your rating. Please try again.')
  }
}

const removeRating = async (contentId: number, type: 'movie' | 'tvShow') => {
  if (!confirm('Are you sure you want to remove this rating?')) return

  try {
    if (type === 'movie') {
      await removeMovieRating(contentId)
      success('Rating Removed', 'Movie rating has been removed')
    } else {
      await removeTvShowRating(contentId)
      success('Rating Removed', 'TV show rating has been removed')
    }

    // Reload the data
    await loadAllRatedMovies()
    await loadAllRatedTvShows()
  } catch (err) {
    console.error('Failed to remove rating:', err)
    error('Remove Failed', 'Failed to remove your rating. Please try again.')
  }
}

onMounted(async () => {
  try {
    await Promise.all([
      loadAllRatedMovies(),
      loadAllRatedTvShows()
    ])
  } catch (err) {
    console.error('Failed to load watched content:', err)
    error('Load Failed', 'Failed to load your watched content. Please try again.')
  }

  // Add keyboard event listeners
  document.addEventListener('keydown', handleKeydown)
})

onUnmounted(() => {
  // Clean up keyboard event listeners
  document.removeEventListener('keydown', handleKeydown)
})
</script>

<style scoped>
.watched-page {
  min-height: 100vh;
  background: var(--color-background);
  padding: var(--spacing-lg);
}

.page-header {
  max-width: 1200px;
  margin: 0 auto var(--spacing-2xl);
}

.header-top {
  display: flex;
  align-items: flex-start;
  gap: var(--spacing-lg);
  margin-bottom: var(--spacing-xl);
}

.back-button {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-sm) var(--spacing-md);
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-medium);
  color: var(--color-text-secondary);
  cursor: pointer;
  transition: all 0.2s ease;
  font-size: 0.875rem;
  font-weight: 500;
  flex-shrink: 0;
}

.back-button:hover {
  background: var(--color-background);
  color: var(--color-text-primary);
  border-color: var(--color-primary);
}

.header-text {
  flex: 1;
  text-align: center;
}

.page-title {
  font-size: 2.5rem;
  font-weight: 700;
  color: var(--color-text-primary);
  margin: 0 0 var(--spacing-sm) 0;
}

.page-subtitle {
  font-size: 1.125rem;
  color: var(--color-text-secondary);
  margin: 0;
}

.search-container {
  max-width: 600px;
  margin: 0 auto;
  margin-bottom: var(--spacing-md);
}

.search-bar {
  position: relative;
  display: flex;
  align-items: center;
}

.search-icon {
  position: absolute;
  left: var(--spacing-md);
  color: var(--color-text-secondary);
  z-index: 1;
  pointer-events: none;
}

.search-input {
  width: 100%;
  padding: var(--spacing-md) var(--spacing-2xl) var(--spacing-md) 48px;
  font-size: 1rem;
  border: 1px solid var(--color-border);
  border-radius: var(--radius-large);
  background-color: var(--color-surface);
  color: var(--color-text-primary);
  transition: all 0.2s ease;
}

.search-input:focus {
  border-color: var(--color-primary);
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
  outline: none;
}

.search-input::placeholder {
  color: var(--color-text-secondary);
}

.clear-button {
  position: absolute;
  right: var(--spacing-md);
  background: none;
  border: none;
  color: var(--color-text-secondary);
  cursor: pointer;
  padding: var(--spacing-xs);
  border-radius: var(--radius-small);
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s ease;
}

.clear-button:hover {
  background: var(--color-background);
  color: var(--color-text-primary);
}

.search-results {
  max-width: 1200px;
  margin: 0 auto var(--spacing-lg);
  text-align: center;
}

.search-results-text {
  color: var(--color-text-secondary);
  font-size: 0.875rem;
  margin: 0;
  padding: var(--spacing-sm) var(--spacing-md);
  background: var(--color-surface);
  border-radius: var(--radius-medium);
  border: 1px solid var(--color-border);
  display: inline-block;
}

.quick-filters {
  max-width: 1200px;
  margin: 0 auto var(--spacing-lg);
  padding: var(--spacing-lg);
  background: var(--color-surface);
  border-radius: var(--radius-large);
  border: 1px solid var(--color-border);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
}

.filter-group {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  flex-wrap: wrap;
}

.filter-label {
  font-size: 1rem;
  font-weight: 600;
  color: var(--color-text-primary);
  margin-right: var(--spacing-md);
  white-space: nowrap;
}

.filter-btn {
  padding: var(--spacing-sm) var(--spacing-md);
  border: 2px solid var(--color-border);
  border-radius: var(--radius-medium);
  background: var(--color-background);
  color: var(--color-text-secondary);
  cursor: pointer;
  transition: all 0.2s ease;
  font-size: 0.875rem;
  font-weight: 600;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.filter-btn:hover {
  background: var(--color-surface);
  color: var(--color-text-primary);
  border-color: var(--color-primary);
}

.filter-btn.active {
  background: var(--color-primary);
  color: white;
  border-color: var(--color-primary);
}

.filter-btn.clear {
  background: #ef4444;
  color: white;
  border-color: #ef4444;
}

.filter-btn.clear:hover {
  background: #dc2626;
  border-color: #dc2626;
}

.content-tabs {
  display: flex;
  gap: var(--spacing-sm);
  max-width: 1200px;
  margin: 0 auto var(--spacing-2xl);
  justify-content: center;
}

.tab-button {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-md) var(--spacing-lg);
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-large);
  color: var(--color-text-secondary);
  cursor: pointer;
  transition: all 0.2s ease;
  font-size: 1rem;
  font-weight: 500;
}

.tab-button:hover {
  background: var(--color-background);
  color: var(--color-text-primary);
}

.tab-button.active {
  background: var(--color-primary);
  color: white;
  border-color: var(--color-primary);
}

.tab-count {
  font-size: 0.8rem;
  opacity: 0.8;
  margin-left: var(--spacing-xs);
}

.loading-container {
  text-align: center;
  padding: var(--spacing-3xl);
  max-width: 1200px;
  margin: 0 auto;
}

.loading-spinner {
  width: 40px;
  height: 40px;
  border: 3px solid var(--color-border);
  border-top: 3px solid var(--color-primary);
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin: 0 auto var(--spacing-lg);
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

.content-list {
  max-width: 1200px;
  margin: 0 auto;
}

.empty-state {
  text-align: center;
  padding: var(--spacing-3xl);
  color: var(--color-text-secondary);
}

.empty-state svg {
  margin-bottom: var(--spacing-lg);
  color: var(--color-text-tertiary);
}

.empty-state h3 {
  font-size: 1.5rem;
  margin: 0 0 var(--spacing-sm) 0;
  color: var(--color-text-primary);
}

.empty-state p {
  margin: 0 0 var(--spacing-lg) 0;
}

.cta-button {
  display: inline-flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-md) var(--spacing-lg);
  background: var(--color-primary);
  color: white;
  text-decoration: none;
  border-radius: var(--radius-medium);
  font-weight: 500;
  transition: background-color 0.2s ease;
}

.cta-button:hover {
  background: var(--color-primary-dark);
}

.content-items {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-lg);
}

.content-item {
  display: flex;
  align-items: center;
  gap: var(--spacing-lg);
  padding: var(--spacing-lg);
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-large);
  cursor: pointer;
  transition: all 0.2s ease;
}

.content-item:hover {
  background: var(--color-background);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  transform: translateY(-2px);
}

.content-poster {
  width: 80px;
  height: 120px;
  border-radius: var(--radius-medium);
  object-fit: cover;
  background: var(--color-background);
  flex-shrink: 0;
}

.content-info {
  flex: 1;
  min-width: 0;
}

.content-title {
  font-size: 1.25rem;
  font-weight: 600;
  color: var(--color-text-primary);
  margin: 0 0 var(--spacing-sm) 0;
  line-height: 1.4;
}

.content-meta {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
}

.rating-display {
  display: flex;
  align-items: center;
}

.watched-date {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  color: var(--color-text-secondary);
  font-size: 0.875rem;
}

.content-actions {
  display: flex;
  gap: var(--spacing-sm);
  flex-shrink: 0;
  opacity: 0;
  transition: opacity 0.2s ease;
}

.content-item:hover .content-actions {
  opacity: 1;
}

.action-button {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 40px;
  height: 40px;
  background: var(--color-background);
  border: 2px solid var(--color-border);
  border-radius: var(--radius-medium);
  cursor: pointer;
  transition: all 0.2s ease;
  color: var(--color-text-secondary);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.action-button:hover {
  background: var(--color-surface);
  color: var(--color-text-primary);
  transform: translateY(-1px);
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.15);
}

.action-button.edit {
  border-color: var(--color-primary);
  color: var(--color-primary);
}

.action-button.edit:hover {
  background: var(--color-primary);
  color: white;
  border-color: var(--color-primary);
}

.action-button.delete {
  border-color: #ef4444;
  color: #ef4444;
}

.action-button.delete:hover {
  background: #ef4444;
  color: white;
  border-color: #ef4444;
}

/* Responsive */
@media (max-width: 768px) {
  .watched-page {
    padding: var(--spacing-md);
  }

  .header-top {
    flex-direction: column;
    align-items: stretch;
    text-align: center;
  }

  .back-button {
    align-self: flex-start;
  }

  .page-title {
    font-size: 2rem;
  }

  .search-input {
    padding: var(--spacing-sm) var(--spacing-xl) var(--spacing-sm) 40px;
    font-size: 0.875rem;
  }

  .search-icon {
    left: var(--spacing-sm);
    width: 18px;
    height: 18px;
  }

  .clear-button {
    right: var(--spacing-sm);
  }

  .quick-filters {
    padding: var(--spacing-md);
    margin-bottom: var(--spacing-md);
  }

  .filter-group {
    flex-direction: column;
    align-items: stretch;
    gap: var(--spacing-md);
  }

  .filter-label {
    text-align: center;
    margin-right: 0;
    margin-bottom: var(--spacing-xs);
  }

  .filter-btn {
    flex: 1;
    min-width: 0;
    text-align: center;
  }

  .content-tabs {
    flex-direction: column;
    align-items: center;
  }

  .tab-button {
    width: 100%;
    max-width: 300px;
    justify-content: center;
  }

  .content-item {
    flex-direction: column;
    align-items: center;
    text-align: center;
  }

  .content-actions {
    opacity: 1;
    margin-top: var(--spacing-sm);
  }

  .content-poster {
    width: 120px;
    height: 180px;
  }

  .content-meta {
    align-items: center;
  }
}

@media (max-width: 480px) {
  .content-item {
    padding: var(--spacing-md);
  }

  .content-poster {
    width: 100px;
    height: 150px;
  }

  .content-title {
    font-size: 1.125rem;
  }
}

/* Search highlighting */
.search-highlight {
  background: rgba(255, 193, 7, 0.3);
  color: var(--color-text-primary);
  font-weight: 600;
  border-radius: 2px;
  padding: 1px 2px;
}

@media (prefers-color-scheme: dark) {
  .search-highlight {
    background: rgba(255, 193, 7, 0.4);
    color: #ffc107;
  }
}
</style>
