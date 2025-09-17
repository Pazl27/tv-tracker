<template>
  <div class="lazy-image-container" :style="{ aspectRatio: aspectRatio }">
    <img
      ref="imageRef"
      :class="[
        'lazy-image',
        {
          'lazy-image--loading': !isLoaded && !hasError,
          'lazy-image--loaded': isLoaded,
          'lazy-image--error': hasError
        }
      ]"
      :alt="alt"
      @load="onLoad"
      @error="onError"
    />
    
    <!-- Loading placeholder -->
    <div v-if="!isLoaded && !hasError" class="lazy-image-placeholder">
      <div v-if="showSpinner" class="loading-spinner"></div>
      <div v-else class="placeholder-background"></div>
    </div>
    
    <!-- Error fallback -->
    <div v-if="hasError" class="lazy-image-error">
      <svg width="32" height="32" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path d="M12 2L15.09 8.26L22 9.27L17 14.14L18.18 21.02L12 17.77L5.82 21.02L7 14.14L2 9.27L8.91 8.26L12 2Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>
      <span class="error-text">Image not available</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch, nextTick, computed } from 'vue'

interface Props {
  src: string
  alt: string
  aspectRatio?: string
  placeholder?: string
  errorImage?: string
  showSpinner?: boolean
  rootMargin?: string
  threshold?: number
  sizes?: string
  quality?: 'low' | 'medium' | 'high'
}

const props = withDefaults(defineProps<Props>(), {
  aspectRatio: '2/3',
  placeholder: '',
  errorImage: '',
  showSpinner: true,
  rootMargin: '100px',
  threshold: 0.1,
  sizes: '(max-width: 768px) 50vw, (max-width: 1200px) 25vw, 20vw',
  quality: 'medium'
})

const emit = defineEmits<{
  load: [event: Event]
  error: [event: Event]
  visible: []
}>()

// Refs
const imageRef = ref<HTMLImageElement>()
const isLoaded = ref(false)
const hasError = ref(false)
const isVisible = ref(false)
const isObserving = ref(false)

// Intersection Observer
let observer: IntersectionObserver | null = null

// Computed optimized image URL
const optimizedSrc = computed(() => {
  if (!props.src) return ''
  
  // If it's already a TMDB URL, optimize the size based on quality
  if (props.src.includes('image.tmdb.org')) {
    const qualityMap = {
      low: 'w300',
      medium: 'w500', 
      high: 'w780'
    }
    
    const size = qualityMap[props.quality]
    return props.src.replace(/w\d+/, size)
  }
  
  return props.src
})

const createObserver = () => {
  if (typeof window === 'undefined' || !('IntersectionObserver' in window)) {
    // Fallback: load immediately if no IntersectionObserver
    loadImage()
    return
  }

  observer = new IntersectionObserver(
    (entries) => {
      entries.forEach((entry) => {
        if (entry.target === imageRef.value && entry.isIntersecting) {
          isVisible.value = true
          emit('visible')
          loadImage()
          observer?.unobserve(entry.target)
        }
      })
    },
    {
      rootMargin: props.rootMargin,
      threshold: props.threshold
    }
  )
}

const loadImage = async () => {
  if (!imageRef.value || !optimizedSrc.value || isLoaded.value || hasError.value) {
    return
  }

  try {
    // Create a temporary image to test loading
    const tempImg = new Image()
    
    await new Promise<void>((resolve, reject) => {
      tempImg.onload = () => resolve()
      tempImg.onerror = () => reject(new Error('Failed to load image'))
      
      // Add sizes for responsive images
      if (props.sizes) {
        tempImg.sizes = props.sizes
      }
      
      tempImg.src = optimizedSrc.value
    })

    // If we get here, the image loaded successfully
    if (imageRef.value) {
      imageRef.value.src = optimizedSrc.value
      if (props.sizes) {
        imageRef.value.sizes = props.sizes
      }
    }
  } catch (error) {
    console.warn('Failed to load image:', optimizedSrc.value, error)
    onError(error as Event)
  }
}

const onLoad = (event: Event) => {
  isLoaded.value = true
  hasError.value = false
  emit('load', event)
}

const onError = (event: Event) => {
  hasError.value = true
  
  // Try fallback image if provided
  if (props.errorImage && imageRef.value && imageRef.value.src !== props.errorImage) {
    imageRef.value.src = props.errorImage
    return
  }
  
  emit('error', event)
}

const startObserving = async () => {
  if (isObserving.value || !imageRef.value) return
  
  await nextTick()
  
  if (observer && imageRef.value) {
    observer.observe(imageRef.value)
    isObserving.value = true
  }
}

const stopObserving = () => {
  if (observer && imageRef.value && isObserving.value) {
    observer.unobserve(imageRef.value)
    isObserving.value = false
  }
}

// Watch for src changes
watch(() => props.src, () => {
  if (isLoaded.value || hasError.value) {
    // Reset state for new image
    isLoaded.value = false
    hasError.value = false
    isVisible.value = false
    
    if (imageRef.value) {
      imageRef.value.src = props.placeholder || ''
    }
    
    // Start observing again
    startObserving()
  }
}, { immediate: false })

onMounted(async () => {
  createObserver()
  await nextTick()
  
  if (!optimizedSrc.value) {
    hasError.value = true
    return
  }
  
  // Set placeholder immediately
  if (imageRef.value && props.placeholder) {
    imageRef.value.src = props.placeholder
  }
  
  startObserving()
})

onUnmounted(() => {
  stopObserving()
  
  if (observer) {
    observer.disconnect()
    observer = null
  }
})
</script>

<style scoped>
.lazy-image-container {
  position: relative;
  overflow: hidden;
  background: var(--color-surface);
  border-radius: var(--radius-medium);
}

.lazy-image {
  width: 100%;
  height: 100%;
  object-fit: cover;
  transition: opacity var(--transition-medium), transform var(--transition-medium);
  display: block;
}

.lazy-image--loading {
  opacity: 0;
}

.lazy-image--loaded {
  opacity: 1;
}

.lazy-image--error {
  opacity: 0;
}

.lazy-image-placeholder {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(
    45deg,
    var(--color-surface) 25%,
    transparent 25%,
    transparent 75%,
    var(--color-surface) 75%
  );
  background-size: 20px 20px;
  animation: shimmer 2s infinite linear;
}

.placeholder-background {
  width: 100%;
  height: 100%;
  background: var(--color-surface);
}

.loading-spinner {
  width: 24px;
  height: 24px;
  border: 2px solid var(--color-border);
  border-top: 2px solid var(--color-accent-primary);
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

.lazy-image-error {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  background: var(--color-surface);
  color: var(--color-text-muted);
  gap: var(--spacing-sm);
}

.error-text {
  font-size: 0.75rem;
  text-align: center;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

@keyframes shimmer {
  0% { background-position: -20px 0; }
  100% { background-position: 20px 0; }
}

/* Responsive optimizations */
@media (max-width: 768px) {
  .loading-spinner {
    width: 20px;
    height: 20px;
  }
  
  .error-text {
    font-size: 0.625rem;
  }
}

/* Reduce motion for accessibility */
@media (prefers-reduced-motion: reduce) {
  .lazy-image {
    transition: opacity 0.2s ease;
  }
  
  .loading-spinner {
    animation: none;
    border: 2px solid var(--color-accent-primary);
  }
  
  .lazy-image-placeholder {
    animation: none;
    background: var(--color-surface);
  }
}

/* High contrast mode support */
@media (prefers-contrast: high) {
  .lazy-image-placeholder {
    background: Canvas;
    border: 1px solid ButtonText;
  }
  
  .lazy-image-error {
    background: Canvas;
    color: ButtonText;
  }
}
</style>