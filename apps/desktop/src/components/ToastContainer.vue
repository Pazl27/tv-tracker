<template>
  <teleport to="body">
    <div class="toast-container">
      <transition-group name="toast" tag="div" class="toast-list">
        <div
          v-for="toast in toasts"
          :key="toast.id"
          :class="['toast', `toast--${toast.type}`]"
          @click="removeToast(toast.id)"
        >
          <div class="toast-icon">
            <!-- Success Icon -->
            <svg v-if="toast.type === 'success'" width="20" height="20" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M9 12L11 14L15 10M21 12C21 16.9706 16.9706 21 12 21C7.02944 21 3 16.9706 3 12C3 7.02944 7.02944 3 12 3C16.9706 3 21 7.02944 21 12Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
            
            <!-- Error Icon -->
            <svg v-else-if="toast.type === 'error'" width="20" height="20" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M12 9V13M12 17H12.01M21 12C21 16.9706 16.9706 21 12 21C7.02944 21 3 16.9706 3 12C3 7.02944 7.02944 3 12 3C16.9706 3 21 7.02944 21 12Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
            
            <!-- Warning Icon -->
            <svg v-else-if="toast.type === 'warning'" width="20" height="20" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M12 9V13M12 17H12.01M10.29 3.86L1.82 18A2 2 0 0 0 3.64 21H20.36A2 2 0 0 0 22.18 18L13.71 3.86A2 2 0 0 0 10.29 3.86Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
            
            <!-- Info Icon -->
            <svg v-else width="20" height="20" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M13 16H12V12H11M12 8H12.01M21 12C21 16.9706 16.9706 21 12 21C7.02944 21 3 16.9706 3 12C3 7.02944 7.02944 3 12 3C16.9706 3 21 7.02944 21 12Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
          </div>
          
          <div class="toast-content">
            <div class="toast-title">{{ toast.title }}</div>
            <div v-if="toast.message" class="toast-message">{{ toast.message }}</div>
          </div>
          
          <button class="toast-close" @click.stop="removeToast(toast.id)">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M18 6L6 18M6 6L18 18" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
          </button>
        </div>
      </transition-group>
    </div>
  </teleport>
</template>

<script setup lang="ts">
import { useToast } from '../composables/useToast'

const { toasts, removeToast } = useToast()
</script>

<style scoped>
.toast-container {
  position: fixed;
  top: var(--spacing-lg);
  right: var(--spacing-lg);
  z-index: 9999;
  pointer-events: none;
}

.toast-list {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
}

.toast {
  display: flex;
  align-items: flex-start;
  gap: var(--spacing-md);
  min-width: 320px;
  max-width: 400px;
  padding: var(--spacing-md);
  border-radius: var(--radius-large);
  backdrop-filter: blur(8px);
  border: 1px solid;
  cursor: pointer;
  pointer-events: auto;
  transition: all var(--transition-medium);
  box-shadow: var(--shadow-large);
}

.toast:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.6);
}

.toast--success {
  background: rgba(35, 134, 54, 0.95);
  border-color: rgba(35, 134, 54, 0.3);
  color: white;
}

.toast--error {
  background: rgba(218, 54, 51, 0.95);
  border-color: rgba(218, 54, 51, 0.3);
  color: white;
}

.toast--warning {
  background: rgba(210, 153, 34, 0.95);
  border-color: rgba(210, 153, 34, 0.3);
  color: white;
}

.toast--info {
  background: rgba(31, 111, 235, 0.95);
  border-color: rgba(31, 111, 235, 0.3);
  color: white;
}

.toast-icon {
  flex-shrink: 0;
  margin-top: 2px;
}

.toast-content {
  flex: 1;
  min-width: 0;
}

.toast-title {
  font-weight: 600;
  font-size: 0.875rem;
  line-height: 1.4;
  margin-bottom: var(--spacing-xs);
}

.toast-message {
  font-size: 0.75rem;
  opacity: 0.9;
  line-height: 1.4;
}

.toast-close {
  flex-shrink: 0;
  background: none;
  border: none;
  color: currentColor;
  cursor: pointer;
  padding: var(--spacing-xs);
  border-radius: var(--radius-small);
  transition: all var(--transition-fast);
  opacity: 0.7;
  min-height: auto;
}

.toast-close:hover {
  opacity: 1;
  background: rgba(255, 255, 255, 0.1);
}

/* Toast Animations */
.toast-enter-active,
.toast-leave-active {
  transition: all var(--transition-medium);
}

.toast-enter-from {
  opacity: 0;
  transform: translateX(100%) scale(0.95);
}

.toast-leave-to {
  opacity: 0;
  transform: translateX(100%) scale(0.95);
}

.toast-move {
  transition: transform var(--transition-medium);
}

/* Responsive Design */
@media (max-width: 480px) {
  .toast-container {
    top: var(--spacing-md);
    right: var(--spacing-md);
    left: var(--spacing-md);
  }
  
  .toast {
    min-width: auto;
    max-width: none;
  }
}
</style>