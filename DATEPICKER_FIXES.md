# Date Picker Fixes Documentation

## Issues Fixed

This document outlines the three main issues that were addressed in the TV Tracker app's date picker functionality.

### 1. Date Selection Off by One Day

**Problem**: When clicking on a date in the calendar, it would select the previous day instead of the clicked date.

**Root Cause**: Using `toISOString()` method which converts dates to UTC timezone, causing timezone offset issues.

**Solution**: 
- Created custom date formatting function that uses local timezone
- Replaced `date.toISOString().split('T')[0]` with `formatDateToLocal(date)`
- Added date validation to ensure selected dates are correct

```typescript
// Before (problematic)
const isoString = date.toISOString().split('T')[0]

// After (fixed)
const dateString = formatDateToLocal(date)
```

### 2. Calendar Popup Positioning

**Problem**: Calendar popup always opened downward, causing it to go outside the viewport and requiring scrolling.

**Solution**:
- Added smart positioning logic that detects available space
- Calendar now opens upward when there's insufficient space below
- Added window resize listener to recalculate position
- Improved mobile positioning with fixed center positioning

```typescript
const updateCalendarPosition = () => {
  const rect = datePickerRef.value.getBoundingClientRect()
  const spaceBelow = viewportHeight - rect.bottom - 20
  const spaceAbove = rect.top - 20
  
  if (spaceBelow < calendarHeight && spaceAbove > calendarHeight) {
    calendarPosition.value = { top: 'auto', bottom: '100%' }
  } else {
    calendarPosition.value = { top: '100%', bottom: 'auto' }
  }
}
```

### 3. Incorrect "Yesterday" Display for Today's Date

**Problem**: Movies/shows rated "today" were showing as "Yesterday" in the watched list.

**Root Cause**: Flawed date comparison logic that didn't properly handle timezone differences and used `Math.ceil()` instead of `Math.floor()`.

**Solution**:
- Reset time components to compare only dates (not time)
- Use `Math.floor()` for accurate day difference calculation
- Added "Today" case (diffDays === 0)

```typescript
// Before (problematic)
const diffTime = Math.abs(now.getTime() - date.getTime())
const diffDays = Math.ceil(diffTime / (1000 * 60 * 60 * 24))

// After (fixed)
const dateOnly = new Date(date.getFullYear(), date.getMonth(), date.getDate())
const nowOnly = new Date(now.getFullYear(), now.getMonth(), now.getDate())
const diffTime = nowOnly.getTime() - dateOnly.getTime()
const diffDays = Math.floor(diffTime / (1000 * 60 * 60 * 24))

if (diffDays === 0) return 'Today'
if (diffDays === 1) return 'Yesterday'
```

## Files Modified

### New Files Created
- `src/components/DatePicker.vue` - Custom date picker component
- `src/utils/dateUtils.ts` - Date utility functions

### Modified Files
- `src/components/RatingPopup.vue` - Integrated custom date picker
- `src/views/WatchedPage.vue` - Updated date formatting logic

## Key Features Added

### Custom DatePicker Component
- **Click-outside closing**: Calendar closes when clicking outside
- **Keyboard navigation**: ESC to close, Enter/Space to open
- **Smart positioning**: Opens above input when space is limited
- **Mobile responsive**: Fixed center positioning on small screens
- **Quick actions**: Today and Clear buttons
- **Date validation**: Prevents invalid date selection

### Date Utility Functions
- `formatDateToLocal()` - Convert Date to YYYY-MM-DD in local timezone
- `formatWatchedDate()` - Display relative dates (Today, Yesterday, etc.)
- `getTodayString()` - Get today's date string
- `getYesterdayString()` - Get yesterday's date string
- Test functions for validation

## Technical Improvements

1. **Timezone Consistency**: All date operations now use local timezone
2. **Error Handling**: Added try-catch blocks for date operations
3. **Performance**: Added debouncing for window resize events
4. **Accessibility**: Proper keyboard navigation and ARIA attributes
5. **Responsive Design**: Optimized for mobile and desktop

## Testing

To test the fixes:

1. **Date Selection**: Click various dates in calendar and verify correct date is selected
2. **Today/Yesterday**: Rate something today and verify it shows "Today" in the watched list
3. **Positioning**: Open calendar near bottom/top of viewport and verify smart positioning
4. **Mobile**: Test on mobile devices for proper responsive behavior
5. **Keyboard**: Use ESC, Enter, and Space keys for navigation

## Browser Compatibility

- Modern browsers with ES2020+ support
- Mobile Safari and Chrome
- Desktop Chrome, Firefox, Safari, Edge