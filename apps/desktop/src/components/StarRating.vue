<template>
  <div class="star-rating" :class="[`size-${size}`, { readonly }]">
    <div class="stars-container">
      <div
        v-for="star in 5"
        :key="star"
        class="star"
        @mouseleave="handleMouseLeave"
      >
        <!-- Left half of star -->
        <div
          class="star-half left"
          @click.stop="handleHalfStarClick(star - 0.5)"
          @mouseover="handleHalfStarHover(star - 0.5)"
          :class="{
            'filled': getStarFillState(star, 'left'),
            'hover': getStarHoverState(star, 'left')
          }"
        >
          <svg viewBox="0 0 24 24" class="star-icon">
            <defs>
              <clipPath id="left-half">
                <rect x="0" y="0" width="12" height="24"/>
              </clipPath>
            </defs>
            <path
              d="M12 2L15.09 8.26L22 9.27L17 14.14L18.18 21.02L12 17.77L5.82 21.02L7 14.14L2 9.27L8.91 8.26L12 2Z"
              clip-path="url(#left-half)"
            />
          </svg>
        </div>
        
        <!-- Right half of star -->
        <div
          class="star-half right"
          @click.stop="handleHalfStarClick(star)"
          @mouseover="handleHalfStarHover(star)"
          :class="{
            'filled': getStarFillState(star, 'right'),
            'hover': getStarHoverState(star, 'right')
          }"
        >
          <svg viewBox="0 0 24 24" class="star-icon">
            <defs>
              <clipPath id="right-half">
                <rect x="12" y="0" width="12" height="24"/>
              </clipPath>
            </defs>
            <path
              d="M12 2L15.09 8.26L22 9.27L17 14.14L18.18 21.02L12 17.77L5.82 21.02L7 14.14L2 9.27L8.91 8.26L12 2Z"
              clip-path="url(#right-half)"
            />
          </svg>
        </div>
      </div>
    </div>
    
    <div v-if="showRatingText" class="rating-text">
      <span v-if="isHovering && hoverRating > 0" class="hover-preview">
        {{ hoverRating.toFixed(1) }}/5.0
      </span>
      <span v-else>
        {{ currentRating > 0 ? `${currentRating.toFixed(1)}/5.0` : 'Not rated' }}
      </span>
    </div>
    
    <button
      v-if="currentRating > 0 && showClearButton"
      @click="clearRating"
      class="clear-rating-btn"
      title="Remove rating"
    >
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path d="M18 6L6 18M6 6L18 18" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>
    </button>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'

interface Props {
  modelValue: number
  readonly?: boolean
  size?: 'small' | 'medium' | 'large'
  showRatingText?: boolean
  showClearButton?: boolean
}

interface Emits {
  (e: 'update:modelValue', value: number): void
  (e: 'change', value: number): void
  (e: 'clear'): void
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: 0,
  readonly: false,
  size: 'medium',
  showRatingText: true,
  showClearButton: true,
})

const emit = defineEmits<Emits>()

const hoverRating = ref(0)
const isHovering = ref(false)

const currentRating = computed(() => props.modelValue)

const displayRating = computed(() => {
  return isHovering.value ? hoverRating.value : currentRating.value
})

const getStarFillState = (starNumber: number, half: 'left' | 'right'): boolean => {
  if (props.readonly) {
    const rating = currentRating.value
    const starValue = half === 'left' ? starNumber - 0.5 : starNumber
    return rating >= starValue
  }
  
  const rating = displayRating.value
  const starValue = half === 'left' ? starNumber - 0.5 : starNumber
  return rating >= starValue
}

const getStarHoverState = (starNumber: number, half: 'left' | 'right'): boolean => {
  if (props.readonly || !isHovering.value) return false
  
  const starValue = half === 'left' ? starNumber - 0.5 : starNumber
  return hoverRating.value >= starValue
}

const handleHalfStarClick = (rating: number) => {
  if (props.readonly) return
  
  emit('update:modelValue', rating)
  emit('change', rating)
}

const handleHalfStarHover = (rating: number) => {
  if (props.readonly) return
  
  isHovering.value = true
  hoverRating.value = rating
}

const handleMouseLeave = () => {
  if (props.readonly) return
  
  isHovering.value = false
  hoverRating.value = 0
}

const clearRating = () => {
  if (props.readonly) return
  
  emit('update:modelValue', 0)
  emit('change', 0)
  emit('clear')
}

// Watch for external changes to modelValue
watch(() => props.modelValue, (newValue) => {
  if (newValue < 0 || newValue > 5) {
    console.warn('Rating value should be between 0 and 5')
  }
})
</script>

<style scoped>
.star-rating {
  display: flex;
  align-items: center;
  gap: 8px;
  user-select: none;
  --star-size: 20px;
}

.stars-container {
  display: flex;
  gap: 2px;
}

.star {
  position: relative;
  display: flex;
  cursor: pointer;
  width: var(--star-size);
  height: var(--star-size);
}

/* Default to medium size */
.star-rating {
  --star-size: 20px;
}

.star-rating.size-small {
  --star-size: 16px;
}

.star-rating.size-medium {
  --star-size: 20px;
}

.star-rating.size-large {
  --star-size: 24px;
}

.star-half {
  position: absolute;
  top: 0;
  width: 50%;
  height: 100%;
  overflow: hidden;
  cursor: pointer;
  transition: all 0.2s ease;
}

.star-half.left {
  left: 0;
}

.star-half.right {
  right: 0;
}

.star-icon {
  width: var(--star-size);
  height: var(--star-size);
  position: absolute;
  top: 0;
  left: 0;
  fill: transparent;
  stroke: #64748b;
  stroke-width: 1;
  transition: all 0.2s ease;
}

.star-half.right .star-icon {
  left: calc(-1 * var(--star-size) / 2);
}

.star-half.filled .star-icon {
  fill: #fbbf24;
  stroke: #f59e0b;
}

.star-half.hover .star-icon {
  fill: #fcd34d;
  stroke: #f59e0b;
  transform: scale(1.1);
}

.star-rating.readonly .star {
  cursor: default;
}

.star-rating.readonly .star-half {
  cursor: default;
}

.rating-text {
  font-size: 14px;
  color: #64748b;
  font-weight: 500;
  min-width: 60px;
}

.hover-preview {
  color: #fbbf24;
  font-weight: 600;
}

.clear-rating-btn {
  background: none;
  border: none;
  cursor: pointer;
  padding: 4px;
  border-radius: 4px;
  color: #64748b;
  transition: all 0.2s ease;
  display: flex;
  align-items: center;
  justify-content: center;
}

.clear-rating-btn:hover {
  background-color: rgba(239, 68, 68, 0.1);
  color: #ef4444;
}

/* Dark theme support */
@media (prefers-color-scheme: dark) {
  .rating-text {
    color: #94a3b8;
  }
  
  .clear-rating-btn {
    color: #94a3b8;
  }
  
  .star-icon {
    stroke: #94a3b8;
  }
}

/* Responsive adjustments */
@media (max-width: 768px) {
  .star-rating {
    gap: 6px;
  }
  
  .stars-container {
    gap: 1px;
  }
  
  .rating-text {
    font-size: 13px;
  }
}
</style>