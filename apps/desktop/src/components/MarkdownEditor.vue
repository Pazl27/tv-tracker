<template>
  <div class="markdown-editor">
    <div class="editor-header">
      <h3 class="editor-title">{{ title }}</h3>
      <div class="editor-tabs">
        <button
          :class="['tab-button', { active: activeTab === 'edit' }]"
          @click="activeTab = 'edit'"
        >
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M11 4H4C2.89543 4 2 4.89543 2 6V20C2 21.1046 2.89543 22 4 22H18C19.1046 22 20 21.1046 20 20V13" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            <path d="M18.5 2.50023C18.8978 2.1024 19.4374 1.87891 20 1.87891C20.5626 1.87891 21.1022 2.1024 21.5 2.50023C21.8978 2.89805 22.1213 3.43762 22.1213 4.00023C22.1213 4.56284 21.8978 5.1024 21.5 5.50023L12 15.0002L8 16.0002L9 12.0002L18.5 2.50023Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
          Edit
        </button>
        <button
          :class="['tab-button', { active: activeTab === 'preview' }]"
          @click="activeTab = 'preview'"
        >
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M1 12S5 4 12 4S23 12 23 12S19 20 12 20S1 12 1 12Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            <circle cx="12" cy="12" r="3" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
          Preview
        </button>
      </div>
    </div>

    <div class="editor-content">
      <!-- Edit Mode -->
      <div v-if="activeTab === 'edit'" class="edit-pane">


        <textarea
          ref="textareaRef"
          v-model="localValue"
          :placeholder="placeholder"
          class="markdown-textarea"
          @keydown="handleKeydown"
          @input="handleInput"
        ></textarea>

        <div class="editor-footer">
          <div class="character-count">
            {{ localValue.length }}/{{ maxLength || '∞' }} characters
          </div>
        </div>
      </div>

      <!-- Preview Mode -->
      <div v-else class="preview-pane">
        <div v-if="!localValue.trim()" class="empty-preview">
          <svg width="48" height="48" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M14 2H6C4.89543 2 4 2.89543 4 4V20C4 21.1046 4.89543 22 6 22H18C19.1046 22 20 21.1046 20 20V8L14 2Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            <polyline points="14,2 14,8 20,8" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
          <p>No content to preview</p>
        </div>
        <div v-else class="preview-content" v-html="renderedHtml"></div>
      </div>
    </div>

    <!-- Action Buttons -->
    <div class="editor-actions" v-if="showActions">
      <button @click="cancelEdit" class="action-btn secondary">
        Cancel
      </button>
      <button @click="saveContent" :disabled="!hasChanges" class="action-btn primary">
        {{ saveButtonText }}
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { ref, computed, watch, nextTick } from 'vue'

interface Props {
  modelValue: string
  title?: string
  placeholder?: string
  maxLength?: number
  showActions?: boolean
  saveButtonText?: string
}

interface Emits {
  (e: 'update:modelValue', value: string): void
  (e: 'save', value: string): void
  (e: 'cancel'): void
}

const props = withDefaults(defineProps<Props>(), {
  title: 'Notes',
  placeholder: 'Write your notes in Markdown...\n\n**Bold text** or *italic text*\n# Headings\n- Lists\n[Links](https://example.com)',
  showActions: true,
  saveButtonText: 'Save Notes'
})

const emit = defineEmits<Emits>()

// Reactive state
const activeTab = ref<'edit' | 'preview'>('edit')
const localValue = ref(props.modelValue)
const textareaRef = ref<HTMLTextAreaElement>()
const renderedHtml = ref('')
const isRendering = ref(false)

// Computed properties
const hasChanges = computed(() => localValue.value !== props.modelValue)

// Methods
const insertMarkdown = (before: string, after: string = '', placeholder: string = '') => {
  if (!textareaRef.value) return

  const textarea = textareaRef.value
  const start = textarea.selectionStart
  const end = textarea.selectionEnd
  const selectedText = localValue.value.substring(start, end)

  let insertText = before
  if (selectedText) {
    insertText += selectedText + after
  } else {
    insertText += placeholder + after
  }

  localValue.value =
    localValue.value.substring(0, start) +
    insertText +
    localValue.value.substring(end)

  // Set cursor position
  nextTick(() => {
    if (selectedText) {
      textarea.selectionStart = start + before.length
      textarea.selectionEnd = start + before.length + selectedText.length
    } else {
      textarea.selectionStart = start + before.length
      textarea.selectionEnd = start + before.length + placeholder.length
    }
    textarea.focus()
  })
}

const handleKeydown = (event: KeyboardEvent) => {
  // Handle keyboard shortcuts
  if (event.ctrlKey || event.metaKey) {
    switch (event.key) {
      case 'b':
        event.preventDefault()
        insertMarkdown('**', '**', 'bold text')
        break
      case 'i':
        event.preventDefault()
        insertMarkdown('*', '*', 'italic text')
        break
      case 's':
        event.preventDefault()
        saveContent()
        break
    }
  }

  // Handle tab for indentation
  if (event.key === 'Tab') {
    event.preventDefault()
    insertMarkdown('  ')
  }
}

const handleInput = () => {
  emit('update:modelValue', localValue.value)
  renderMarkdown()
}

const renderMarkdown = async () => {
  if (isRendering.value || !localValue.value.trim()) {
    renderedHtml.value = ''
    return
  }

  isRendering.value = true
  try {
    const html = await invoke('render_markdown_to_html', {
      markdown: localValue.value
    }) as string
    renderedHtml.value = html
  } catch (error) {
    console.error('Failed to render markdown:', error)
    renderedHtml.value = `<p style="color: var(--color-error);">Failed to render markdown preview</p>`
  } finally {
    isRendering.value = false
  }
}



const saveContent = () => {
  emit('save', localValue.value)
}

const cancelEdit = () => {
  localValue.value = props.modelValue
  emit('cancel')
}

// Watch for external changes
watch(() => props.modelValue, (newValue) => {
  localValue.value = newValue
  renderMarkdown()
}, { immediate: true })

// Watch for tab changes to render preview
watch(activeTab, (tab) => {
  if (tab === 'preview') {
    renderMarkdown()
  }
})
</script>

<style scoped>
.markdown-editor {
  display: flex;
  flex-direction: column;
  background: var(--color-card-background);
  border: 1px solid var(--color-card-border);
  border-radius: var(--radius-large);
  overflow: hidden;
  min-height: 300px;
}

.editor-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-md);
  border-bottom: 1px solid var(--color-border);
  background: var(--color-surface);
}

.editor-title {
  font-size: 1.125rem;
  font-weight: 600;
  color: var(--color-text-primary);
  margin: 0;
}

.editor-tabs {
  display: flex;
  background: var(--color-background);
  border-radius: var(--radius-medium);
  padding: 2px;
}

.tab-button {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  padding: var(--spacing-xs) var(--spacing-md);
  background: transparent;
  border: none;
  border-radius: var(--radius-small);
  color: var(--color-text-secondary);
  font-size: 0.875rem;
  font-weight: 500;
  cursor: pointer;
  transition: all var(--transition-fast);
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

.editor-content {
  flex: 1;
  display: flex;
  flex-direction: column;
}

.edit-pane {
  flex: 1;
  display: flex;
  flex-direction: column;
}



.markdown-textarea {
  flex: 1;
  padding: var(--spacing-md);
  border: none;
  background: var(--color-background);
  color: var(--color-text-primary);
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-size: 0.875rem;
  line-height: 1.6;
  resize: none;
  min-height: 200px;
}

.markdown-textarea:focus {
  outline: none;
}

.markdown-textarea::placeholder {
  color: var(--color-text-muted);
  line-height: 1.6;
}

.editor-footer {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  padding: var(--spacing-sm) var(--spacing-md);
  background: var(--color-surface);
  border-top: 1px solid var(--color-border);
  font-size: 0.75rem;
  color: var(--color-text-muted);
}



.character-count {
  color: var(--color-text-secondary);
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
}

.preview-pane {
  flex: 1;
  padding: var(--spacing-md);
  background: var(--color-background);
}

.empty-preview {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  text-align: center;
  color: var(--color-text-muted);
  height: 100%;
  min-height: 200px;
}

.empty-preview svg {
  margin-bottom: var(--spacing-md);
  opacity: 0.5;
}

.empty-preview p {
  margin: var(--spacing-xs) 0;
}

.preview-content {
  color: var(--color-text-primary);
  line-height: 1.6;
}

/* Markdown preview styles */
.preview-content :deep(h1),
.preview-content :deep(h2),
.preview-content :deep(h3),
.preview-content :deep(h4),
.preview-content :deep(h5),
.preview-content :deep(h6) {
  color: var(--color-text-primary);
  margin-top: var(--spacing-lg);
  margin-bottom: var(--spacing-md);
  font-weight: 600;
}

.preview-content :deep(h1) {
  font-size: 1.5rem;
  border-bottom: 2px solid var(--color-border);
  padding-bottom: var(--spacing-sm);
}

.preview-content :deep(h2) {
  font-size: 1.375rem;
  border-bottom: 1px solid var(--color-border);
  padding-bottom: var(--spacing-xs);
}

.preview-content :deep(h3) {
  font-size: 1.25rem;
}

.preview-content :deep(p) {
  margin-bottom: var(--spacing-md);
}

.preview-content :deep(ul),
.preview-content :deep(ol) {
  margin-bottom: var(--spacing-md);
  padding-left: var(--spacing-lg);
}

.preview-content :deep(li) {
  margin-bottom: var(--spacing-xs);
}

.preview-content :deep(strong) {
  font-weight: 600;
  color: var(--color-text-primary);
}

.preview-content :deep(em) {
  font-style: italic;
  color: var(--color-text-primary);
}

.preview-content :deep(a) {
  color: var(--color-accent-secondary);
  text-decoration: none;
  border-bottom: 1px solid transparent;
  transition: border-color var(--transition-fast);
}

.preview-content :deep(a:hover) {
  border-bottom-color: var(--color-accent-secondary);
}

.preview-content :deep(code) {
  background: var(--color-surface);
  padding: 2px 4px;
  border-radius: 3px;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-size: 0.875em;
  color: var(--color-text-primary);
}

.preview-content :deep(blockquote) {
  border-left: 4px solid var(--color-accent-primary);
  padding: var(--spacing-md);
  margin: var(--spacing-md) 0;
  background: var(--color-surface);
  border-radius: 0 var(--radius-medium) var(--radius-medium) 0;
}

.editor-actions {
  display: flex;
  gap: var(--spacing-md);
  justify-content: flex-end;
  padding: var(--spacing-md);
  background: var(--color-surface);
  border-top: 1px solid var(--color-border);
}

.action-btn {
  padding: var(--spacing-sm) var(--spacing-lg);
  border-radius: var(--radius-medium);
  font-size: 0.875rem;
  font-weight: 500;
  cursor: pointer;
  transition: all var(--transition-fast);
  border: 1px solid transparent;
  min-height: auto;
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
  background: var(--color-accent-primary);
  color: white;
}

.action-btn.primary:hover:not(:disabled) {
  background: var(--color-accent-primary-hover);
}

.action-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

/* Responsive */
@media (max-width: 768px) {
  .editor-header {
    flex-direction: column;
    gap: var(--spacing-md);
    align-items: stretch;
  }

  .editor-tabs {
    align-self: stretch;
  }

  .tab-button {
    flex: 1;
    justify-content: center;
  }



  .editor-footer {
    flex-direction: column;
    gap: var(--spacing-xs);
    align-items: stretch;
  }



  .editor-actions {
    flex-direction: column;
  }

  .action-btn {
    width: 100%;
  }
}
</style>
