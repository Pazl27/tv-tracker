<template>
  <div class="tabbar-container">
    <!-- Header with Home and Search -->
    <div class="header-section">
      <!-- Home Link -->
      <div class="home-link" @click="goHome">
        <svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
          <path d="M3 12L5 10M5 10L12 3L19 10M5 10V20C5 20.5523 5.44772 21 6 21H9M19 10L21 12M19 10V20C19 20.5523 18.5523 21 18 21H15M9 21C9.55228 21 10 20.5523 10 20V16C10 15.4477 10.4477 15 11 15H13C13.5523 15 14 15.4477 14 16V20C14 20.5523 14.4477 21 15 21M9 21H15" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
        <span>Home</span>
      </div>

      <!-- Search Bar -->
      <div class="search-container">
        <div class="search-bar">
          <svg class="search-icon" width="20" height="20" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M21 21L16.514 16.506L21 21ZM19 10.5C19 15.194 15.194 19 10.5 19C5.806 19 2 15.194 2 10.5C2 5.806 5.806 2 10.5 2C15.194 2 19 5.806 19 10.5Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
          <input
            v-model="searchQuery"
            type="text"
            placeholder="Search movies and TV shows..."
            class="search-input"
            @keydown.enter="onSearchInput"
            @input="onSearchInput"
          />
          <button
            v-if="searchQuery"
            type="button"
            class="clear-button"
            @click="clearSearch"
          >
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M18 6L6 18M6 6L18 18" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
          </button>
        </div>
      </div>
    </div>

    <!-- Sub Tabs -->
    <div class="sub-tabbar">
      <div class="tab-wrapper">
        <button
          :class="['tab-button', { active: activeSubTab === 'movies' }]"
          @click="switchSubTab('movies')"
        >
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M7 4V2C7 1.44772 7.44772 1 8 1H16C16.5523 1 17 1.44772 17 2V4M7 4H5C3.89543 4 3 4.89543 3 6V20C3 21.1046 3.89543 22 5 22H19C20.1046 22 21 21.1046 21 20V6C21 4.89543 20.1046 4 19 4H17M7 4H17M9 9H15M9 13H15M9 17H15" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
          <span>Movies</span>
        </button>

        <button
          :class="['tab-button', { active: activeSubTab === 'tvShows' }]"
          @click="switchSubTab('tvShows')"
        >
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M7 21H17C18.1046 21 19 20.1046 19 19V5C19 3.89543 18.1046 3 17 3H7C5.89543 3 5 3.89543 5 5V19C5 20.1046 5.89543 21 7 21ZM9 9H15V15H9V9Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
          <span>TV Shows</span>
        </button>
      </div>

      <div class="tab-indicator" :class="{ 'tab-indicator--tv': activeSubTab === 'tvShows' }"></div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { defineEmits, ref } from 'vue';
import { useRouter } from 'vue-router';

const emit = defineEmits<{
  (e: 'sub-tab-switched', subTab: string): void;
  (e: 'search-input', query: string): void;
}>();

withDefaults(defineProps<{
  activeSubTab?: string;
}>(), {
  activeSubTab: 'movies'
});

const searchQuery = ref('');
const router = useRouter();

const switchSubTab = (subTab: string) => {
  emit('sub-tab-switched', subTab);
};

const onSearchInput = () => {
  emit('search-input', searchQuery.value);
};

const clearSearch = () => {
  searchQuery.value = '';
  emit('search-input', '');
};

const goHome = () => {
  router.push('/');
};
</script>

<style scoped>
.tabbar-container {
  background: var(--color-background-secondary);
  border-bottom: 1px solid var(--color-border);
  padding: var(--spacing-lg) var(--spacing-md);
  position: sticky;
  top: 0;
  z-index: 100;
  backdrop-filter: blur(8px);
}

.header-section {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--spacing-lg);
  margin-bottom: var(--spacing-lg);
  max-width: 1400px;
  margin-left: auto;
  margin-right: auto;
}

.home-link {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  cursor: pointer;
  font-size: 1.125rem;
  font-weight: 600;
  color: var(--color-text-primary);
  padding: var(--spacing-sm) var(--spacing-md);
  border-radius: var(--radius-large);
  transition: all var(--transition-fast);
  background: var(--color-surface);
  border: 1px solid var(--color-border);
}

.home-link:hover {
  background: var(--color-surface-hover);
  border-color: var(--color-accent-primary);
  color: var(--color-accent-primary);
  transform: translateY(-1px);
}

.search-container {
  flex: 1;
  max-width: 600px;
}

.search-bar {
  position: relative;
  display: flex;
  align-items: center;
}

.search-icon {
  position: absolute;
  left: var(--spacing-md);
  color: var(--color-text-muted);
  z-index: 1;
  pointer-events: none;
}

.search-input {
  width: 100%;
  padding: var(--spacing-md) var(--spacing-2xl) var(--spacing-md) 48px;
  font-size: 1rem;
  border: 2px solid var(--color-border);
  border-radius: var(--radius-xl);
  background-color: var(--color-background);
  color: var(--color-text-primary);
  transition: all var(--transition-medium);
  box-shadow: var(--shadow-small);
}

.search-input:focus {
  border-color: var(--color-accent-secondary);
  box-shadow: 0 0 0 3px rgba(31, 111, 235, 0.1), var(--shadow-medium);
  outline: none;
}

.search-input::placeholder {
  color: var(--color-text-muted);
}

.clear-button {
  position: absolute;
  right: var(--spacing-md);
  background: none;
  border: none;
  color: var(--color-text-muted);
  cursor: pointer;
  padding: var(--spacing-xs);
  border-radius: var(--radius-small);
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all var(--transition-fast);
  min-height: auto;
}

.clear-button:hover {
  background: var(--color-surface);
  color: var(--color-text-primary);
}

.sub-tabbar {
  position: relative;
  display: flex;
  justify-content: center;
  max-width: 400px;
  margin: 0 auto;
}

.tab-wrapper {
  display: flex;
  background: var(--color-background);
  border-radius: var(--radius-large);
  padding: var(--spacing-xs);
  border: 1px solid var(--color-border);
  position: relative;
}

.tab-button {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-md) var(--spacing-lg);
  border: none;
  background: transparent;
  color: var(--color-text-secondary);
  font-size: 0.875rem;
  font-weight: 500;
  cursor: pointer;
  border-radius: var(--radius-medium);
  transition: all var(--transition-medium);
  position: relative;
  z-index: 2;
  min-height: auto;
}

.tab-button:hover {
  color: var(--color-text-primary);
  background: var(--color-surface);
}

.tab-button.active {
  color: var(--color-text-primary);
  background: var(--color-accent-primary);
}

.tab-button.active svg {
  color: currentColor;
}

.tab-indicator {
  position: absolute;
  top: var(--spacing-xs);
  left: var(--spacing-xs);
  height: calc(100% - 8px);
  width: calc(50% - 4px);
  background: var(--color-accent-primary);
  border-radius: var(--radius-medium);
  transition: transform var(--transition-medium);
  z-index: 1;
}

.tab-indicator--tv {
  transform: translateX(100%);
}

/* Responsive design */
@media (max-width: 768px) {
  .header-section {
    flex-direction: column;
    gap: var(--spacing-md);
  }

  .home-link {
    align-self: flex-start;
  }

  .search-container {
    max-width: 100%;
  }

  .tab-button span {
    display: none;
  }

  .tab-button {
    padding: var(--spacing-md);
  }

  .tab-wrapper {
    max-width: 120px;
  }
}

@media (max-width: 480px) {
  .tabbar-container {
    padding: var(--spacing-md) var(--spacing-sm);
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
}
</style>
