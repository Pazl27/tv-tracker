<template>
  <div class="date-picker" ref="datePickerRef">
    <label v-if="label" :for="inputId" class="date-label">{{ label }}</label>
    <div class="date-input-container">
      <input
        :id="inputId"
        ref="inputRef"
        type="text"
        :value="displayValue"
        @click="toggleCalendar"
        @focus="showCalendar"
        @keydown="handleKeydown"
        :placeholder="placeholder"
        class="date-input"
        readonly
      />
      <svg class="calendar-icon" width="16" height="16" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path d="M8 2V6M16 2V6M3 10H21M5 4H19C20.1046 4 21 4.89543 21 6V20C21 21.1046 20.1046 22 19 22H5C3.89543 22 4 21.1046 4 20V6C4 4.89543 3.89543 4 5 4Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>
    </div>
    
    <div v-if="isCalendarVisible" class="calendar-popup" @click.stop>
      <div class="calendar-header">
        <button @click="previousMonth" class="nav-button" type="button">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M15 18L9 12L15 6" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        </button>
        <span class="month-year">{{ monthYearDisplay }}</span>
        <button @click="nextMonth" class="nav-button" type="button">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M9 18L15 12L9 6" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        </button>
      </div>
      
      <div class="calendar-grid">
        <div class="weekday-headers">
          <div v-for="day in weekdays" :key="day" class="weekday-header">{{ day }}</div>
        </div>
        
        <div class="days-grid">
          <button
            v-for="day in calendarDays"
            :key="`${day.date.getTime()}`"
            @click="selectDate(day.date)"
            :class="[
              'day-button',
              {
                'other-month': !day.isCurrentMonth,
                'selected': isSelectedDate(day.date),
                'today': isToday(day.date),
                'disabled': isDisabled(day.date)
              }
            ]"
            :disabled="isDisabled(day.date)"
            type="button"
          >
            {{ day.date.getDate() }}
          </button>
        </div>
      </div>
      
      <div class="calendar-footer">
        <button @click="selectToday" class="quick-select-btn" type="button">Today</button>
        <button @click="clearDate" class="quick-select-btn clear" type="button">Clear</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted, nextTick } from 'vue'
import { formatDateToLocal } from '../utils/dateUtils'

interface Props {
  modelValue?: string | Date | null
  label?: string
  placeholder?: string
  maxDate?: string | Date
  minDate?: string | Date
  disabled?: boolean
}

interface Emits {
  (e: 'update:modelValue', value: string | null): void
  (e: 'change', value: string | null): void
}

const props = withDefaults(defineProps<Props>(), {
  placeholder: 'Select a date',
  disabled: false
})

const emit = defineEmits<Emits>()

const datePickerRef = ref<HTMLElement>()
const inputRef = ref<HTMLInputElement>()
const isCalendarVisible = ref(false)
const currentMonth = ref(new Date().getMonth())
const currentYear = ref(new Date().getFullYear())
const calendarPosition = ref({ top: '100%', bottom: 'auto' })

const inputId = computed(() => `date-picker-${Math.random().toString(36).substr(2, 9)}`)

const weekdays = ['Su', 'Mo', 'Tu', 'We', 'Th', 'Fr', 'Sa']

const selectedDate = computed(() => {
  if (!props.modelValue) return null
  if (typeof props.modelValue === 'string') {
    return new Date(props.modelValue)
  }
  return props.modelValue
})

const displayValue = computed(() => {
  if (!selectedDate.value) return ''
  return selectedDate.value.toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric'
  })
})

const monthYearDisplay = computed(() => {
  const date = new Date(currentYear.value, currentMonth.value)
  return date.toLocaleDateString('en-US', {
    month: 'long',
    year: 'numeric'
  })
})

const calendarDays = computed(() => {
  const firstDay = new Date(currentYear.value, currentMonth.value, 1)
  const startDate = new Date(firstDay.getTime())
  startDate.setDate(startDate.getDate() - firstDay.getDay())
  
  const days = []
  const currentDate = new Date(startDate.getTime())
  
  // Generate 42 days (6 weeks)
  for (let i = 0; i < 42; i++) {
    days.push({
      date: new Date(currentDate.getTime()),
      isCurrentMonth: currentDate.getMonth() === currentMonth.value
    })
    currentDate.setDate(currentDate.getDate() + 1)
  }
  
  return days
})

const isSelectedDate = (date: Date) => {
  if (!selectedDate.value) return false
  return (
    date.getDate() === selectedDate.value.getDate() &&
    date.getMonth() === selectedDate.value.getMonth() &&
    date.getFullYear() === selectedDate.value.getFullYear()
  )
}

const isToday = (date: Date) => {
  const today = new Date()
  return (
    date.getDate() === today.getDate() &&
    date.getMonth() === today.getMonth() &&
    date.getFullYear() === today.getFullYear()
  )
}

const isDisabled = (date: Date) => {
  if (props.disabled) return true
  
  if (props.maxDate) {
    const maxDate = typeof props.maxDate === 'string' ? new Date(props.maxDate) : props.maxDate
    if (date > maxDate) return true
  }
  
  if (props.minDate) {
    const minDate = typeof props.minDate === 'string' ? new Date(props.minDate) : props.minDate
    if (date < minDate) return true
  }
  
  return false
}

const toggleCalendar = async () => {
  if (props.disabled) return
  isCalendarVisible.value = !isCalendarVisible.value
  if (isCalendarVisible.value) {
    await nextTick()
    updateCalendarPosition()
  }
}

const showCalendar = async () => {
  if (props.disabled) return
  isCalendarVisible.value = true
  await nextTick()
  updateCalendarPosition()
}

const hideCalendar = () => {
  isCalendarVisible.value = false
}

const selectDate = (date: Date) => {
  if (isDisabled(date)) return
  
  try {
    // Create date string in local timezone to avoid timezone offset issues
    const dateString = formatDateToLocal(date)
    
    // Validate the date string
    const testDate = new Date(dateString + 'T00:00:00')
    if (isNaN(testDate.getTime())) {
      console.warn('Invalid date selected:', dateString)
      return
    }
    
    emit('update:modelValue', dateString)
    emit('change', dateString)
    hideCalendar()
  } catch (error) {
    console.error('Error selecting date:', error)
  }
}

const selectToday = () => {
  try {
    const today = new Date()
    // Reset to start of day to avoid timezone issues
    const todayLocal = new Date(today.getFullYear(), today.getMonth(), today.getDate())
    if (!isDisabled(todayLocal)) {
      selectDate(todayLocal)
    }
  } catch (error) {
    console.error('Error selecting today:', error)
  }
}

const clearDate = () => {
  emit('update:modelValue', null)
  emit('change', null)
  hideCalendar()
}

const previousMonth = () => {
  if (currentMonth.value === 0) {
    currentMonth.value = 11
    currentYear.value--
  } else {
    currentMonth.value--
  }
}

const nextMonth = () => {
  if (currentMonth.value === 11) {
    currentMonth.value = 0
    currentYear.value++
  } else {
    currentMonth.value++
  }
}

const updateCalendarPosition = () => {
  if (!datePickerRef.value) return
  
  try {
    // On mobile, use fixed positioning
    if (window.innerWidth <= 640) {
      calendarPosition.value = { top: '50%', bottom: 'auto' }
      return
    }
    
    const rect = datePickerRef.value.getBoundingClientRect()
    const viewportHeight = window.innerHeight
    const calendarHeight = 400 // Approximate calendar height
    const spaceBelow = viewportHeight - rect.bottom - 20 // Add some padding
    const spaceAbove = rect.top - 20 // Add some padding
    
    if (spaceBelow < calendarHeight && spaceAbove > calendarHeight) {
      // Show above input
      calendarPosition.value = { top: 'auto', bottom: '100%' }
    } else {
      // Show below input (default)
      calendarPosition.value = { top: '100%', bottom: 'auto' }
    }
  } catch (error) {
    console.error('Error updating calendar position:', error)
    // Fallback to default position
    calendarPosition.value = { top: '100%', bottom: 'auto' }
  }
}

const handleKeydown = (event: KeyboardEvent) => {
  if (event.key === 'Escape') {
    hideCalendar()
    inputRef.value?.blur()
  } else if (event.key === 'Enter' || event.key === ' ') {
    event.preventDefault()
    toggleCalendar()
  }
}

const handleClickOutside = (event: Event) => {
  if (datePickerRef.value && !datePickerRef.value.contains(event.target as Node)) {
    hideCalendar()
  }
}

// Initialize current month/year based on selected date
watch(() => props.modelValue, (newValue) => {
  if (newValue) {
    const date = typeof newValue === 'string' ? new Date(newValue) : newValue
    currentMonth.value = date.getMonth()
    currentYear.value = date.getFullYear()
  }
}, { immediate: true })

onMounted(() => {
  document.addEventListener('click', handleClickOutside)
  window.addEventListener('resize', updateCalendarPosition)
})

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside)
  window.removeEventListener('resize', updateCalendarPosition)
})
</script>

<style scoped>
.date-picker {
  position: relative;
  width: 100%;
}

.date-label {
  display: block;
  font-size: 0.875rem;
  font-weight: 500;
  color: var(--color-text-secondary);
  margin-bottom: 6px;
}

.date-input-container {
  position: relative;
  display: flex;
  align-items: center;
}

.date-input {
  width: 100%;
  padding: 12px 40px 12px 16px;
  border: 1px solid var(--color-border);
  border-radius: 8px;
  background: var(--color-background);
  color: var(--color-text-primary);
  font-size: 1rem;
  cursor: pointer;
  transition: border-color 0.2s ease;
}

.date-input:hover {
  border-color: var(--color-primary);
}

.date-input:focus {
  outline: none;
  border-color: var(--color-primary);
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.date-input:disabled {
  background: var(--color-surface);
  color: var(--color-text-secondary);
  cursor: not-allowed;
}

.calendar-icon {
  position: absolute;
  right: 12px;
  color: var(--color-text-secondary);
  pointer-events: none;
}

.calendar-popup {
  position: absolute;
  top: v-bind('calendarPosition.top');
  bottom: v-bind('calendarPosition.bottom');
  left: 0;
  right: 0;
  margin-top: 4px;
  background: var(--color-background);
  border: 1px solid var(--color-border);
  border-radius: 12px;
  box-shadow: 0 10px 40px rgba(0, 0, 0, 0.15);
  z-index: 1000;
  padding: 16px;
  min-width: 280px;
  max-height: 400px;
  overflow-y: auto;
}

.calendar-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 16px;
}

.nav-button {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border: none;
  background: var(--color-surface);
  border-radius: 8px;
  color: var(--color-text-secondary);
  cursor: pointer;
  transition: all 0.2s ease;
}

.nav-button:hover {
  background: var(--color-primary);
  color: white;
}

.month-year {
  font-weight: 600;
  color: var(--color-text-primary);
  font-size: 1rem;
}

.calendar-grid {
  margin-bottom: 16px;
}

.weekday-headers {
  display: grid;
  grid-template-columns: repeat(7, 1fr);
  gap: 2px;
  margin-bottom: 8px;
}

.weekday-header {
  text-align: center;
  font-size: 0.75rem;
  font-weight: 500;
  color: var(--color-text-secondary);
  padding: 8px 4px;
}

.days-grid {
  display: grid;
  grid-template-columns: repeat(7, 1fr);
  gap: 2px;
}

.day-button {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border: none;
  background: transparent;
  border-radius: 6px;
  color: var(--color-text-primary);
  cursor: pointer;
  transition: all 0.2s ease;
  font-size: 0.875rem;
}

.day-button:hover:not(.disabled) {
  background: var(--color-surface);
}

.day-button.other-month {
  color: var(--color-text-secondary);
  opacity: 0.5;
}

.day-button.selected {
  background: var(--color-primary);
  color: white;
}

.day-button.today:not(.selected) {
  background: var(--color-primary);
  color: white;
  opacity: 0.7;
}

.day-button.disabled {
  color: var(--color-text-secondary);
  opacity: 0.3;
  cursor: not-allowed;
}

.calendar-footer {
  display: flex;
  gap: 8px;
  justify-content: space-between;
  border-top: 1px solid var(--color-border);
  padding-top: 12px;
}

.quick-select-btn {
  padding: 6px 12px;
  border: 1px solid var(--color-border);
  background: var(--color-surface);
  color: var(--color-text-secondary);
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.2s ease;
  font-size: 0.875rem;
}

.quick-select-btn:hover {
  background: var(--color-primary);
  color: white;
  border-color: var(--color-primary);
}

.quick-select-btn.clear:hover {
  background: #ef4444;
  border-color: #ef4444;
}

/* Dark theme support */
@media (prefers-color-scheme: dark) {
  .calendar-popup {
    background: #1f2937;
    border-color: #374151;
    box-shadow: 0 10px 40px rgba(0, 0, 0, 0.3);
  }
  
  .date-input {
    background: #111827;
    border-color: #374151;
    color: #f9fafb;
  }
  
  .date-input:focus {
    border-color: #3b82f6;
  }
}

/* When positioned above */
.calendar-popup[style*="bottom: 100%"] {
  margin-top: 0;
  margin-bottom: 4px;
}

/* Mobile responsive */
@media (max-width: 640px) {
  .calendar-popup {
    position: fixed;
    top: 50% !important;
    bottom: auto !important;
    left: 50%;
    right: auto;
    transform: translate(-50%, -50%);
    margin: 0;
    max-width: 320px;
    width: 90vw;
    max-height: 80vh;
    overflow-y: auto;
  }
}

/* Very small screens */
@media (max-width: 480px) {
  .calendar-popup {
    width: 95vw;
    max-width: 300px;
    padding: 12px;
  }
  
  .day-button {
    width: 28px;
    height: 28px;
    font-size: 0.75rem;
  }
  
  .calendar-header {
    margin-bottom: 12px;
  }
  
  .month-year {
    font-size: 0.9rem;
  }
}
</style>