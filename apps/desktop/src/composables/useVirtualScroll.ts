import { ref, computed, onMounted, onUnmounted, nextTick, Ref } from 'vue'

export interface VirtualScrollOptions {
  itemHeight: number
  containerHeight?: number
  overscan?: number
  threshold?: number
}

export const useVirtualScroll = <T>(
  items: Ref<T[]>,
  options: VirtualScrollOptions
) => {
  const {
    itemHeight,
    containerHeight = 600,
    overscan = 5,
    threshold = 0.1
  } = options

  // Refs
  const containerRef = ref<HTMLElement>()
  const scrollTop = ref(0)
  const isScrolling = ref(false)
  const clientHeight = ref(containerHeight)

  let scrollEndTimer: NodeJS.Timeout | null = null

  // Computed values
  const totalHeight = computed(() => items.value.length * itemHeight)
  
  const visibleItemsCount = computed(() => 
    Math.ceil(clientHeight.value / itemHeight) + overscan * 2
  )
  
  const startIndex = computed(() => 
    Math.max(0, Math.floor(scrollTop.value / itemHeight) - overscan)
  )
  
  const endIndex = computed(() => 
    Math.min(
      items.value.length - 1,
      startIndex.value + visibleItemsCount.value - 1
    )
  )
  
  const visibleItems = computed(() => {
    const start = startIndex.value
    const end = endIndex.value
    
    return items.value.slice(start, end + 1).map((item, index) => ({
      item,
      index: start + index,
      top: (start + index) * itemHeight
    }))
  })
  
  const offsetY = computed(() => startIndex.value * itemHeight)

  // Methods
  const handleScroll = (event: Event) => {
    const target = event.target as HTMLElement
    scrollTop.value = target.scrollTop
    isScrolling.value = true

    // Debounce scroll end detection
    if (scrollEndTimer) {
      clearTimeout(scrollEndTimer)
    }
    
    scrollEndTimer = setTimeout(() => {
      isScrolling.value = false
    }, 150)
  }

  const scrollToIndex = (index: number, behavior: ScrollBehavior = 'smooth') => {
    if (!containerRef.value) return
    
    const top = index * itemHeight
    containerRef.value.scrollTo({
      top,
      behavior
    })
  }

  const scrollToTop = (behavior: ScrollBehavior = 'smooth') => {
    scrollToIndex(0, behavior)
  }

  const scrollToBottom = (behavior: ScrollBehavior = 'smooth') => {
    scrollToIndex(items.value.length - 1, behavior)
  }

  const getItemOffset = (index: number) => {
    return index * itemHeight
  }

  const isItemVisible = (index: number) => {
    return index >= startIndex.value && index <= endIndex.value
  }

  // Resize observer for dynamic container height
  const setupResizeObserver = () => {
    if (!containerRef.value || !('ResizeObserver' in window)) {
      return
    }

    const observer = new ResizeObserver((entries) => {
      for (const entry of entries) {
        clientHeight.value = entry.contentRect.height
      }
    })

    observer.observe(containerRef.value)
    return observer
  }

  // Lifecycle
  let resizeObserver: ResizeObserver | null = null

  onMounted(async () => {
    await nextTick()
    
    if (containerRef.value) {
      clientHeight.value = containerRef.value.clientHeight
      resizeObserver = setupResizeObserver()
    }
  })

  onUnmounted(() => {
    if (scrollEndTimer) {
      clearTimeout(scrollEndTimer)
    }
    
    if (resizeObserver) {
      resizeObserver.disconnect()
    }
  })

  return {
    // Refs
    containerRef,
    scrollTop,
    isScrolling,
    clientHeight,
    
    // Computed
    totalHeight,
    visibleItemsCount,
    startIndex,
    endIndex,
    visibleItems,
    offsetY,
    
    // Methods
    handleScroll,
    scrollToIndex,
    scrollToTop,
    scrollToBottom,
    getItemOffset,
    isItemVisible
  }
}

// Helper hook for infinite scrolling
export const useInfiniteScroll = <T>(
  items: Ref<T[]>,
  loadMore: () => Promise<void>,
  options: VirtualScrollOptions & {
    hasMore?: Ref<boolean>
    isLoading?: Ref<boolean>
    loadMoreThreshold?: number
  } = {}
) => {
  const {
    hasMore = ref(true),
    isLoading = ref(false),
    loadMoreThreshold = 0.8,
    ...virtualScrollOptions
  } = options

  const virtualScroll = useVirtualScroll(items, virtualScrollOptions)
  
  const handleScroll = async (event: Event) => {
    virtualScroll.handleScroll(event)
    
    if (isLoading.value || !hasMore.value) return
    
    const target = event.target as HTMLElement
    const scrollPercentage = 
      (target.scrollTop + target.clientHeight) / target.scrollHeight
    
    if (scrollPercentage >= loadMoreThreshold) {
      isLoading.value = true
      try {
        await loadMore()
      } finally {
        isLoading.value = false
      }
    }
  }

  return {
    ...virtualScroll,
    handleScroll,
    hasMore,
    isLoading
  }
}