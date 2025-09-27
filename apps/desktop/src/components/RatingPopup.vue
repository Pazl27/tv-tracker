<template>
  <div v-if="isVisible" class="rating-popup-overlay" @click="handleOverlayClick">
    <div class="rating-popup" @click.stop>
      <div class="popup-header">
        <h3 class="popup-title">Rate {{ contentType }}</h3>
        <button class="close-button" @click="closePopup">
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M18 6L6 18M6 6L18 18" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        </button>
      </div>

      <div class="popup-content">
        <!-- Content Info -->
        <div class="content-info">
          <img 
            :src="content.poster_url || `https://image.tmdb.org/t/p/w200${content.poster_path}`"
            :alt="content.title || content.name"
            class="content-poster"
            @error="handleImageError" 
          />
          <div class="content-details">
            <h4 class="content-title">{{ content.title || content.name }}</h4>
            <p class="content-year">
              {{ contentType === 'Movie' ? getYear(content.release_date) : getYear(content.first_air_date) }}
            </p>
          </div>
        </div>

        <!-- Rating Section -->
        <div class="rating-section">
          <label class="section-label">Your Rating</label>
          <StarRating
            v-model="rating"
            :show-rating-text="true"
            :show-clear-button="false"
            size="large"
            @change="handleRatingChange"
          />
        </div>

        <!-- Date/Time Section -->
        <div class="datetime-section">
          <label class="section-label">When did you watch this?</label>
          <div class="datetime-inputs">
            <div class="input-group">
              <DatePicker
                v-model="watchDate"
                label="Date"
                placeholder="Select watch date"
                :max-date="today"
                @change="handleDateChange"
              />
            </div>
            <div class="input-group">
              <label for="watch-time" class="input-label">Time (optional)</label>
              <input
                id="watch-time"
                v-model="watchTime"
                type="time"
                class="time-input"
              />
            </div>
          </div>
        </div>

        <!-- Quick Date Buttons -->
        <div class="quick-dates">
          <button class="quick-date-btn" @click="setQuickDate('today')">Today</button>
          <button class="quick-date-btn" @click="setQuickDate('yesterday')">Yesterday</button>
          <button class="quick-date-btn" @click="setQuickDate('week')">This Week</button>
        </div>

        <!-- Action Buttons -->
        <div class="popup-actions">
          <button class="action-btn secondary" @click="closePopup">Cancel</button>
          <button 
            :disabled="!rating || rating === 0" 
            class="action-btn primary" 
            @click="saveRating"
          >
            {{ existingRating ? 'Update Rating' : 'Save Rating' }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import DatePicker from './DatePicker.vue'
import StarRating from './StarRating.vue'
import { formatDateToLocal, getTodayString } from '../utils/dateUtils'
import { ref, computed, watch, onMounted } from 'vue'

interface Props {
  isVisible: boolean
  content: any
  contentType: 'Movie' | 'TV Show'
  existingRating?: number
  existingWatchedAt?: string
}

interface Emits {
  (e: 'close'): void
  (e: 'save', data: { rating: number; watchedAt: string }): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const rating = ref(0)
const watchDate = ref('')
const watchTime = ref('')

const today = computed(() => {
  return getTodayString()
})

const handleOverlayClick = () => {
  closePopup()
}

const closePopup = () => {
  emit('close')
}

const handleRatingChange = (newRating: number) => {
  rating.value = newRating
}

const handleDateChange = (newDate: string | null) => {
  watchDate.value = newDate || ''
}

const handleImageError = (event: Event) => {
  const target = event.target as HTMLImageElement
  target.src = '/placeholder-poster.png' // You can add a placeholder image
}

const getYear = (dateString: string) => {
  if (!dateString) return 'Unknown'
  return new Date(dateString).getFullYear()
}

const setQuickDate = (period: string) => {
  const now = new Date()

  switch (period) {
    case 'today': {
      watchDate.value = formatDateToLocal(now)
      watchTime.value = now.toTimeString().slice(0, 5)
      break
    }
    case 'yesterday': {
      const yesterday = new Date(now)
      yesterday.setDate(yesterday.getDate() - 1)
      watchDate.value = formatDateToLocal(yesterday)
      watchTime.value = '20:00' // Default evening time
      break
    }
    case 'week': {
      const weekAgo = new Date(now)
      weekAgo.setDate(weekAgo.getDate() - 7)
      watchDate.value = formatDateToLocal(weekAgo)
      watchTime.value = '20:00'
      break
    }
  }
}

const saveRating = () => {
  if (!rating.value || rating.value === 0) return

  // Combine date and time into ISO string
  let watchedAt: string

  if (watchDate.value) {
    if (watchTime.value) {
      watchedAt = new Date(`${watchDate.value}T${watchTime.value}`).toISOString()
    } else {
      watchedAt = new Date(`${watchDate.value}T20:00:00`).toISOString()
    }
  } else {
    watchedAt = new Date().toISOString()
  }

  emit('save', {
    rating: rating.value,
    watchedAt
  })
}

// Initialize with existing data if editing
watch(() => props.isVisible, (isVisible) => {
  if (isVisible) {
    // Reset or load existing data
    rating.value = props.existingRating || 0

    if (props.existingWatchedAt) {
      const existingDate = new Date(props.existingWatchedAt)
      watchDate.value = formatDateToLocal(existingDate)
      watchTime.value = existingDate.toTimeString().slice(0, 5)
    } else {
      // Default to today
      const now = new Date()
      watchDate.value = formatDateToLocal(now)
      watchTime.value = now.toTimeString().slice(0, 5)
    }
  }
})

onMounted(() => {
  // Set default date to today
  const now = new Date()
  watchDate.value = formatDateToLocal(now)
  watchTime.value = now.toTimeString().slice(0, 5)
})
</script>

<style scoped>
.rating-popup-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.7);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  padding: 20px;
}

.rating-popup {
  background: var(--color-background);
  border-radius: 16px;
  max-width: 500px;
  width: 100%;
  max-height: 90vh;
  overflow-y: auto;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.4);
  border: 1px solid var(--color-border);
}

.popup-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 24px 24px 0 24px;
  border-bottom: 1px solid var(--color-border);
  margin-bottom: 24px;
}

.popup-title {
  font-size: 1.5rem;
  font-weight: 600;
  color: var(--color-text-primary);
  margin: 0;
}

.close-button {
  background: none;
  border: none;
  cursor: pointer;
  padding: 8px;
  border-radius: 8px;
  color: var(--color-text-secondary);
  transition: all 0.2s ease;
}

.close-button:hover {
  background: var(--color-surface);
  color: var(--color-text-primary);
}

.popup-content {
  padding: 0 24px 24px;
}

.content-info {
  display: flex;
  gap: 16px;
  margin-bottom: 32px;
  align-items: center;
}

.content-poster {
  width: 80px;
  height: 120px;
  border-radius: 8px;
  object-fit: cover;
  background: var(--color-surface);
}

.content-details {
  flex: 1;
}

.content-title {
  font-size: 1.25rem;
  font-weight: 600;
  color: var(--color-text-primary);
  margin: 0 0 4px 0;
  line-height: 1.4;
}

.content-year {
  color: var(--color-text-secondary);
  margin: 0;
  font-size: 0.9rem;
}

.rating-section {
  margin-bottom: 32px;
}

.section-label {
  display: block;
  font-size: 1rem;
  font-weight: 600;
  color: var(--color-text-primary);
  margin-bottom: 12px;
}

.datetime-section {
  margin-bottom: 24px;
}

.datetime-inputs {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
}

.input-group {
  display: flex;
  flex-direction: column;
}

.input-label {
  font-size: 0.875rem;
  font-weight: 500;
  color: var(--color-text-secondary);
  margin-bottom: 6px;
}

.time-input {
  padding: 12px 16px;
  border: 1px solid var(--color-border);
  border-radius: 8px;
  background: var(--color-background);
  color: var(--color-text-primary);
  font-size: 1rem;
  transition: border-color 0.2s ease;
}

.time-input:focus {
  outline: none;
  border-color: var(--color-primary);
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.quick-dates {
  display: flex;
  gap: 8px;
  margin-bottom: 32px;
  flex-wrap: wrap;
}

.quick-date-btn {
  padding: 8px 16px;
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: 20px;
  color: var(--color-text-secondary);
  font-size: 0.875rem;
  cursor: pointer;
  transition: all 0.2s ease;
}

.quick-date-btn:hover {
  background: var(--color-primary);
  color: white;
  border-color: var(--color-primary);
}

.popup-actions {
  display: flex;
  gap: 12px;
  justify-content: flex-end;
}

.action-btn {
  padding: 12px 24px;
  border-radius: 8px;
  font-size: 1rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  border: 1px solid transparent;
}

.action-btn.secondary {
  background: var(--color-surface);
  color: var(--color-text-secondary);
  border-color: var(--color-border);
}

.action-btn.secondary:hover {
  background: var(--color-background);
  color: var(--color-text-primary);
}

.action-btn.primary {
  background: var(--color-primary);
  color: white;
}

.action-btn.primary:hover {
  background: var(--color-primary-dark);
}

.action-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.action-btn.primary:disabled:hover {
  background: var(--color-primary);
}

/* Dark theme support */
@media (prefers-color-scheme: dark) {
  .rating-popup {
    background: #1f2937;
    border-color: #374151;
  }

  .time-input {
    background: #111827;
    border-color: #374151;
    color: #f9fafb;
  }

  .time-input:focus {
    border-color: #3b82f6;
  }
}

/* Mobile responsive */
@media (max-width: 640px) {
  .rating-popup {
    margin: 0;
    border-radius: 16px 16px 0 0;
    max-height: 80vh;
  }

  .datetime-inputs {
    grid-template-columns: 1fr;
  }

  .popup-actions {
    flex-direction: column;
  }

  .action-btn {
    width: 100%;
  }
}
</style>
