/**
 * Date utility functions for consistent date handling across the application
 */

/**
 * Formats a date to YYYY-MM-DD string in local timezone
 */
export function formatDateToLocal(date: Date): string {
  const year = date.getFullYear()
  const month = String(date.getMonth() + 1).padStart(2, '0')
  const day = String(date.getDate()).padStart(2, '0')
  return `${year}-${month}-${day}`
}

/**
 * Formats a watched date for display (Today, Yesterday, X days ago, etc.)
 */
export function formatWatchedDate(dateString: string): string {
  try {
    const date = new Date(dateString)
    const now = new Date()

    // Reset time to compare only dates
    const dateOnly = new Date(date.getFullYear(), date.getMonth(), date.getDate())
    const nowOnly = new Date(now.getFullYear(), now.getMonth(), now.getDate())

    const diffTime = nowOnly.getTime() - dateOnly.getTime()
    const diffDays = Math.floor(diffTime / (1000 * 60 * 60 * 24))

    if (diffDays === 0) return 'Today'
    if (diffDays === 1) return 'Yesterday'
    if (diffDays <= 7) return `${diffDays} days ago`
    if (diffDays <= 30) return `${Math.ceil(diffDays / 7)} weeks ago`
    if (diffDays <= 365) return `${Math.ceil(diffDays / 30)} months ago`

    return date.toLocaleDateString('en-US', {
      year: 'numeric',
      month: 'short',
      day: 'numeric'
    })
  } catch (e) {
    return 'Unknown date'
  }
}

/**
 * Gets today's date in YYYY-MM-DD format
 */
export function getTodayString(): string {
  return formatDateToLocal(new Date())
}

/**
 * Gets yesterday's date in YYYY-MM-DD format
 */
export function getYesterdayString(): string {
  const yesterday = new Date()
  yesterday.setDate(yesterday.getDate() - 1)
  return formatDateToLocal(yesterday)
}

/**
 * Gets a date N days ago in YYYY-MM-DD format
 */
export function getDaysAgoString(days: number): string {
  const date = new Date()
  date.setDate(date.getDate() - days)
  return formatDateToLocal(date)
}

/**
 * Parses a date string and returns a Date object
 */
export function parseDate(dateString: string): Date | null {
  try {
    const date = new Date(dateString)
    return isNaN(date.getTime()) ? null : date
  } catch {
    return null
  }
}

/**
 * Checks if a date string represents today
 */
export function isToday(dateString: string): boolean {
  return dateString === getTodayString()
}

/**
 * Checks if a date string represents yesterday
 */
export function isYesterday(dateString: string): boolean {
  return dateString === getYesterdayString()
}

/**
 * Test function to verify date logic
 */
export function testDateLogic(): void {

  const today = getTodayString()
  const yesterday = getYesterdayString()
  const weekAgo = getDaysAgoString(7)

}
