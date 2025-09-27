<template>
  <div class="notes-section">
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

      <div class="section-actions" v-if="!isEditing">
        <button
          @click="startEditing"
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
    <div v-if="isLoading" class="loading-state">
      <div class="loading-spinner"></div>
      <p>Loading notes...</p>
    </div>

    <!-- Error State -->
    <div v-else-if="error" class="error-state">
      <svg width="48" height="48" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
        <circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="2"/>
        <line x1="15" y1="9" x2="9" y2="15" stroke="currentColor" stroke-width="2"/>
        <line x1="9" y1="9" x2="15" y2="15" stroke="currentColor" stroke-width="2"/>
      </svg>
      <h4>Failed to load notes</h4>
      <p>{{ error }}</p>
      <button @click="loadNotes" class="retry-btn">Try Again</button>
    </div>

    <!-- Edit Mode -->
    <div v-else-if="isEditing" class="edit-mode">
      <MarkdownEditor
        v-model="editingNotes"
        :title="`Notes for ${contentTitle}`"
        :placeholder="placeholder"
        :max-length="5000"
        :show-actions="true"
        :save-button-text="hasNotes ? 'Update Notes' : 'Save Notes'"
        @save="saveNotes"
        @cancel="cancelEditing"
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
        <p>Add your personal notes, thoughts, or reminders about {{ contentTitle }}.</p>
        <button @click="startEditing" class="add-notes-btn">
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
          <span class="last-updated">Last updated: {{ formatDate(lastUpdated) }}</span>
          <div class="notes-actions">
            <button @click="startEditing" class="action-btn small edit-btn">
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
</template>

<script setup lang="ts">
import MarkdownEditor from './MarkdownEditor.vue'
import { useToast } from '../composables/useToast'
import { invoke } from '@tauri-apps/api/core'
import { ref, computed, onMounted, watch } from 'vue'

interface Props {
  contentId: number
  contentType: 'movie' | 'tv-show' | 'watched-movie' | 'watched-tv-show'
  contentTitle: string
}

const props = defineProps<Props>()
const { success, error: showError } = useToast()

// Reactive state
const notes = ref('')
const editingNotes = ref('')
const renderedNotes = ref('')
const isEditing = ref(false)
const isLoading = ref(false)
const isSaving = ref(false)
const error = ref('')
const lastUpdated = ref<Date | null>(null)

// Computed
const hasNotes = computed(() => notes.value.trim().length > 0)

const placeholder = computed(() => {
  const type = props.contentType.includes('movie') ? 'movie' : 'TV show'
  return `Write your thoughts about this ${type}...\n\n**What did you think?**\n- Great story\n- Amazing performances\n- Would watch again\n\n*Use markdown for formatting!*`
})

// Methods
const getNotesCommand = () => {
  const commandMap = {
    'movie': 'get_movie_notes',
    'tv-show': 'get_tv_show_notes',
    'watched-movie': 'get_watched_movie_notes',
    'watched-tv-show': 'get_watched_tv_show_notes'
  }
  return commandMap[props.contentType]
}

const updateNotesCommand = () => {
  const commandMap = {
    'movie': 'update_movie_notes',
    'tv-show': 'update_tv_show_notes',
    'watched-movie': 'update_watched_movie_notes',
    'watched-tv-show': 'update_watched_tv_show_notes'
  }
  return commandMap[props.contentType]
}

const getCommandParams = () => {
  if (props.contentType.includes('movie')) {
    return { movieId: props.contentId }
  } else {
    return { tvShowId: props.contentId }
  }
}

const loadNotes = async () => {
  if (isLoading.value) return

  isLoading.value = true
  error.value = ''

  try {
    const command = getNotesCommand()
    const params = getCommandParams()

    const result = await invoke(command, params) as string | null
    notes.value = result || ''

    if (notes.value) {
      await renderNotes()
      lastUpdated.value = new Date() // In a real app, this would come from the backend
    }
  } catch (err) {
    console.error('Failed to load notes:', err)
    error.value = 'Failed to load notes. Please try again.'
  } finally {
    isLoading.value = false
  }
}

const renderNotes = async () => {
  if (!notes.value.trim()) {
    renderedNotes.value = ''
    return
  }

  try {
    const html = await invoke('render_markdown_to_html', {
      markdown: notes.value
    }) as string
    renderedNotes.value = html
  } catch (err) {
    console.error('Failed to render notes:', err)
    renderedNotes.value = `<p style="color: var(--color-error);">Failed to render notes</p>`
  }
}

const startEditing = () => {
  editingNotes.value = notes.value
  isEditing.value = true
}

const cancelEditing = () => {
  editingNotes.value = ''
  isEditing.value = false
}

const saveNotes = async (newNotes: string) => {
  if (isSaving.value) return

  isSaving.value = true

  try {
    const command = updateNotesCommand()
    const params = {
      ...getCommandParams(),
      notes: newNotes
    }

    await invoke(command, params)

    notes.value = newNotes
    lastUpdated.value = new Date()
    isEditing.value = false

    if (newNotes.trim()) {
      await renderNotes()
      success('Notes Saved', 'Your notes have been saved successfully')
    } else {
      renderedNotes.value = ''
      success('Notes Cleared', 'Your notes have been cleared')
    }
  } catch (err) {
    console.error('Failed to save notes:', err)
    showError('Save Failed', 'Failed to save your notes. Please try again.')
  } finally {
    isSaving.value = false
  }
}

const clearNotes = async () => {
  if (!confirm('Are you sure you want to clear all notes? This action cannot be undone.')) {
    return
  }

  await saveNotes('')
}

const formatDate = (date: Date | null) => {
  if (!date) return 'Unknown'

  const now = new Date()
  const diffMs = now.getTime() - date.getTime()
  const diffMins = Math.floor(diffMs / (1000 * 60))
  const diffHours = Math.floor(diffMs / (1000 * 60 * 60))
  const diffDays = Math.floor(diffMs / (1000 * 60 * 60 * 24))

  if (diffMins < 1) return 'Just now'
  if (diffMins < 60) return `${diffMins} minute${diffMins === 1 ? '' : 's'} ago`
  if (diffHours < 24) return `${diffHours} hour${diffHours === 1 ? '' : 's'} ago`
  if (diffDays < 7) return `${diffDays} day${diffDays === 1 ? '' : 's'} ago`

  return date.toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric'
  })
}

// Watch for content changes
watch(() => [props.contentId, props.contentType], () => {
  loadNotes()
}, { immediate: false })

// Load notes on mount
onMounted(() => {
  loadNotes()
})
</script>

<style scoped>
.notes-section {
  background: var(--color-card-background);
  border: 1px solid var(--color-card-border);
  border-radius: var(--radius-large);
  overflow: hidden;
  margin-top: var(--spacing-lg);
}

.section-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-md);
  background: var(--color-surface);
  border-bottom: 1px solid var(--color-border);
}

.section-title {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  font-size: 1.125rem;
  font-weight: 600;
  color: var(--color-text-primary);
  margin: 0;
}

.section-actions {
  display: flex;
  gap: var(--spacing-sm);
}

.action-btn {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  padding: var(--spacing-xs) var(--spacing-md);
  background: var(--color-accent-primary);
  color: white;
  border: none;
  border-radius: var(--radius-medium);
  font-size: 0.875rem;
  font-weight: 500;
  cursor: pointer;
  transition: all var(--transition-fast);
  min-height: auto;
}

.action-btn:hover {
  background: var(--color-accent-primary-hover);
  transform: translateY(-1px);
}

.action-btn.small {
  padding: 4px var(--spacing-sm);
  font-size: 0.75rem;
}

.action-btn.edit-btn {
  background: var(--color-accent-primary);
}

.action-btn.delete-btn {
  background: var(--color-error);
}

.action-btn.delete-btn:hover {
  background: #dc2626;
}

.loading-state,
.error-state,
.empty-notes {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  text-align: center;
  padding: var(--spacing-2xl);
  color: var(--color-text-muted);
}

.loading-spinner {
  width: 32px;
  height: 32px;
  border: 3px solid var(--color-border);
  border-top: 3px solid var(--color-accent-primary);
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin-bottom: var(--spacing-md);
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

.error-state svg,
.empty-notes svg {
  margin-bottom: var(--spacing-md);
  opacity: 0.5;
}

.error-state h4,
.empty-notes h4 {
  color: var(--color-text-primary);
  margin-bottom: var(--spacing-sm);
}

.error-state p,
.empty-notes p {
  margin-bottom: var(--spacing-lg);
}

.retry-btn,
.add-notes-btn {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  padding: var(--spacing-sm) var(--spacing-lg);
  background: var(--color-accent-primary);
  color: white;
  border: none;
  border-radius: var(--radius-medium);
  font-weight: 500;
  cursor: pointer;
  transition: all var(--transition-fast);
  min-height: auto;
}

.retry-btn:hover,
.add-notes-btn:hover {
  background: var(--color-accent-primary-hover);
}

.edit-mode {
  padding: 0;
}

.display-mode {
  padding: 0;
}

.notes-display {
  padding: var(--spacing-md);
}

.notes-content {
  color: var(--color-text-primary);
  line-height: 1.6;
  margin-bottom: var(--spacing-md);
}

/* Markdown content styles */
.notes-content :deep(h1),
.notes-content :deep(h2),
.notes-content :deep(h3),
.notes-content :deep(h4),
.notes-content :deep(h5),
.notes-content :deep(h6) {
  color: var(--color-text-primary);
  margin-top: var(--spacing-lg);
  margin-bottom: var(--spacing-md);
  font-weight: 600;
}

.notes-content :deep(h1) {
  font-size: 1.5rem;
  border-bottom: 2px solid var(--color-border);
  padding-bottom: var(--spacing-sm);
}

.notes-content :deep(h2) {
  font-size: 1.375rem;
  border-bottom: 1px solid var(--color-border);
  padding-bottom: var(--spacing-xs);
}

.notes-content :deep(h3) {
  font-size: 1.25rem;
}

.notes-content :deep(p) {
  margin-bottom: var(--spacing-md);
}

.notes-content :deep(ul),
.notes-content :deep(ol) {
  margin-bottom: var(--spacing-md);
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
  color: var(--color-text-primary);
}

.notes-content :deep(a) {
  color: var(--color-accent-secondary);
  text-decoration: none;
  border-bottom: 1px solid transparent;
  transition: border-color var(--transition-fast);
}

.notes-content :deep(a:hover) {
  border-bottom-color: var(--color-accent-secondary);
}

.notes-content :deep(code) {
  background: var(--color-surface);
  padding: 2px 4px;
  border-radius: 3px;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-size: 0.875em;
  color: var(--color-text-primary);
}

.notes-content :deep(blockquote) {
  border-left: 4px solid var(--color-accent-primary);
  padding: var(--spacing-md);
  margin: var(--spacing-md) 0;
  background: var(--color-surface);
  border-radius: 0 var(--radius-medium) var(--radius-medium) 0;
}

.notes-meta {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding-top: var(--spacing-md);
  border-top: 1px solid var(--color-border);
  font-size: 0.75rem;
  color: var(--color-text-muted);
}

.notes-actions {
  display: flex;
  gap: var(--spacing-xs);
}

/* Responsive */
@media (max-width: 768px) {
  .section-header {
    flex-direction: column;
    gap: var(--spacing-md);
    align-items: stretch;
  }

  .notes-meta {
    flex-direction: column;
    gap: var(--spacing-sm);
    align-items: stretch;
  }

  .notes-actions {
    justify-content: center;
  }

  .loading-state,
  .error-state,
  .empty-notes {
    padding: var(--spacing-lg);
  }
}
</style>
