import { ref, onMounted, onUnmounted, nextTick } from 'vue'

export interface LazyImageOptions {
  rootMargin?: string
  threshold?: number
  placeholder?: string
  errorImage?: string
}

export const useLazyImage = (options: LazyImageOptions = {}) => {
  const {
    rootMargin = '50px',
    threshold = 0.1,
    placeholder = '',
    errorImage = ''
  } = options

  // State management
  const imageElements = ref(new Map<string, HTMLImageElement>())
  const loadedImages = ref(new Set<string>())
  const errorImages = ref(new Set<string>())
  const visibleImages = ref(new Set<string>())
  
  let observer: IntersectionObserver | null = null

  // Create intersection observer
  const createObserver = () => {
    if (typeof window === 'undefined' || !('IntersectionObserver' in window)) {
      return null
    }

    return new IntersectionObserver(
      (entries) => {
        entries.forEach((entry) => {
          const img = entry.target as HTMLImageElement
          const imageId = img.dataset.imageId
          
          if (!imageId) return

          if (entry.isIntersecting) {
            visibleImages.value.add(imageId)
            loadImage(imageId, img)
            observer?.unobserve(img)
          }
        })
      },
      {
        rootMargin,
        threshold
      }
    )
  }

  // Load image when it becomes visible
  const loadImage = (imageId: string, imgElement: HTMLImageElement) => {
    const src = imgElement.dataset.src
    if (!src) return

    // Create a new image to preload
    const img = new Image()
    
    img.onload = () => {
      loadedImages.value.add(imageId)
      imgElement.src = src
      imgElement.classList.add('loaded')
    }
    
    img.onerror = () => {
      errorImages.value.add(imageId)
      if (errorImage) {
        imgElement.src = errorImage
      }
      imgElement.classList.add('error')
    }
    
    img.src = src
  }

  // Register an image for lazy loading
  const registerImage = async (element: HTMLImageElement, src: string, imageId: string) => {
    if (!element || !src || !imageId) return

    // Set up the element
    element.dataset.src = src
    element.dataset.imageId = imageId
    element.src = placeholder || 'data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMSIgaGVpZ2h0PSIxIiB2aWV3Qm94PSIwIDAgMSAxIiBmaWxsPSJub25lIiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciPjxyZWN0IHdpZHRoPSIxIiBoZWlnaHQ9IjEiIGZpbGw9IiNmMGYwZjAiLz48L3N2Zz4='
    element.classList.add('lazy-image')
    
    // Store reference
    imageElements.value.set(imageId, element)

    // Wait for next tick to ensure DOM is ready
    await nextTick()

    // If no intersection observer, load immediately
    if (!observer) {
      visibleImages.value.add(imageId)
      loadImage(imageId, element)
      return
    }

    // Start observing
    observer.observe(element)
  }

  // Unregister an image
  const unregisterImage = (imageId: string) => {
    const element = imageElements.value.get(imageId)
    if (element && observer) {
      observer.unobserve(element)
    }
    
    imageElements.value.delete(imageId)
    loadedImages.value.delete(imageId)
    errorImages.value.delete(imageId)
    visibleImages.value.delete(imageId)
  }

  // Check if image is loaded
  const isImageLoaded = (imageId: string) => {
    return loadedImages.value.has(imageId)
  }

  // Check if image has error
  const hasImageError = (imageId: string) => {
    return errorImages.value.has(imageId)
  }

  // Check if image is visible
  const isImageVisible = (imageId: string) => {
    return visibleImages.value.has(imageId)
  }

  // Preload images that are likely to be needed soon
  const preloadImage = (src: string) => {
    const img = new Image()
    img.src = src
  }

  // Initialize
  onMounted(() => {
    observer = createObserver()
  })

  // Cleanup
  onUnmounted(() => {
    if (observer) {
      observer.disconnect()
      observer = null
    }
    
    imageElements.value.clear()
    loadedImages.value.clear()
    errorImages.value.clear()
    visibleImages.value.clear()
  })

  return {
    registerImage,
    unregisterImage,
    isImageLoaded,
    hasImageError,
    isImageVisible,
    preloadImage,
    loadedImages: loadedImages,
    errorImages: errorImages,
    visibleImages: visibleImages
  }
}