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
        <div class="toolbar">
          <div class="toolbar-group">
            <button
              class="toolbar-btn"
              @click="insertMarkdown('**', '**', 'bold text')"
              title="Bold (Ctrl+B)"
            >
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M6 12H9.5C10.8807 12 12 10.8807 12 9.5V9.5C12 8.11929 10.8807 7 9.5 7H6V12ZM6 12H10.5C11.8807 12 13 13.1193 13 14.5V14.5C13 15.8807 11.8807 17 10.5 17H6V12Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
            </button>
            <button
              class="toolbar-btn"
              @click="insertMarkdown('*', '*', 'italic text')"
              title="Italic (Ctrl+I)"
            >
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <line x1="19" y1="4" x2="10" y2="4" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                <line x1="14" y1="20" x2="5" y2="20" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                <line x1="15" y1="4" x2="9" y2="20" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
            </button>
          </div>

          <div class="toolbar-group">
            <button
              class="toolbar-btn"
              @click="insertMarkdown('## ', '', 'Heading')"
              title="Heading"
            >
              H
            </button>
            <button
              class="toolbar-btn"
              @click="insertMarkdown('- ', '', 'List item')"
              title="List"
            >
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <line x1="8" y1="6" x2="21" y2="6" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                <line x1="8" y1="12" x2="21" y2="12" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                <line x1="8" y1="18" x2="21" y2="18" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                <line x1="3" y1="6" x2="3.01" y2="6" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                <line x1="3" y1="12" x2="3.01" y2="12" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                <line x1="3" y1="18" x2="3.01" y2="18" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
            </button>
            <button
              class="toolbar-btn"
              @click="insertMarkdown('[', '](url)', 'Link text')"
              title="Link"
            >
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M10 13C10.4295 13.5741 10.9774 14.0491 11.6066 14.3929C12.2357 14.7367 12.9315 14.9411 13.6467 14.9923C14.3618 15.0435 15.0796 14.9403 15.7513 14.6897C16.4231 14.4392 17.0331 14.047 17.54 13.54L20.54 10.54C21.4508 9.59695 21.9548 8.33394 21.9434 7.02296C21.932 5.71198 21.4061 4.45791 20.4791 3.53087C19.5521 2.60383 18.298 2.07799 16.987 2.0666C15.676 2.0552 14.413 2.55918 13.47 3.47L11.75 5.18" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                <path d="M14 11C13.5705 10.4259 13.0226 9.95088 12.3934 9.60707C11.7643 9.26327 11.0685 9.05892 10.3533 9.00771C9.63819 8.9565 8.92037 9.05973 8.24864 9.31026C7.5769 9.56079 6.96687 9.95302 6.46 10.46L3.46 13.46C2.54917 14.403 2.04519 15.6661 2.05659 16.977C2.06798 18.288 2.59382 19.5421 3.52086 20.4691C4.44791 21.3962 5.70198 21.922 7.01296 21.9334C8.32394 21.9448 9.58695 21.4408 10.53 20.53L12.24 18.82" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
            </button>
          </div>

          <div class="toolbar-group">
            <button
              v-if="modelValue"
              class="toolbar-btn clear-btn"
              @click="clearContent"
              title="Clear all content"
            >
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <polyline points="3,6 5,6 21,6" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                <path d="M19,6V20C19,21.1046 18.1046,22 17,22H7C5.89543,22 5,21.1046 5,20V6M8,6V4C8,2.89543 8.89543,2 10,2H14C15.1046,2 16,2.89543 16,4V6" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
            </button>
          </div>
        </div>

        <textarea
          ref="textareaRef"
          v-model="localValue"
          :placeholder="placeholder"
          class="markdown-textarea"
          @keydown="handleKeydown"
          @input="handleInput"
        ></textarea>

        <div class="editor-footer">
          <div class="markdown-help">
            <span class="help-text">
              Supports <strong>**bold**</strong>, <em>*italic*</em>,
              <code># headings</code>, <code>- lists</code>, and <code>[links](url)</code>
            </span>
          </div>
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
          <p class="help-text">Switch to Edit mode to start writing your notes</p>
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

const clearContent = () => {
  if (confirm('Are you sure you want to clear all content? This cannot be undone.')) {
    localValue.value = ''
    emit('update:modelValue', '')
    renderedHtml.value = ''
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

.toolbar {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-sm) var(--spacing-md);
  background: var(--color-surface);
  border-bottom: 1px solid var(--color-border);
}

.toolbar-group {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
}

.toolbar-group:not(:last-child)::after {
  content: '';
  width: 1px;
  height: 16px;
  background: var(--color-border);
  margin-left: var(--spacing-sm);
}

.toolbar-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  background: transparent;
  border: 1px solid transparent;
  border-radius: var(--radius-small);
  color: var(--color-text-secondary);
  cursor: pointer;
  transition: all var(--transition-fast);
  font-size: 0.875rem;
  font-weight: 600;
  min-height: auto;
}

.toolbar-btn:hover {
  background: var(--color-background);
  border-color: var(--color-border);
  color: var(--color-text-primary);
}

.toolbar-btn.clear-btn:hover {
  background: var(--color-error);
  border-color: var(--color-error);
  color: white;
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
  justify-content: space-between;
  padding: var(--spacing-sm) var(--spacing-md);
  background: var(--color-surface);
  border-top: 1px solid var(--color-border);
  font-size: 0.75rem;
  color: var(--color-text-muted);
}

.markdown-help {
  flex: 1;
}

.help-text {
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

  .toolbar {
    flex-wrap: wrap;
    gap: var(--spacing-xs);
  }

  .toolbar-group::after {
    display: none;
  }

  .editor-footer {
    flex-direction: column;
    gap: var(--spacing-xs);
    align-items: stretch;
  }

  .markdown-help {
    text-align: center;
  }

  .editor-actions {
    flex-direction: column;
  }

  .action-btn {
    width: 100%;
  }
}
</style>
