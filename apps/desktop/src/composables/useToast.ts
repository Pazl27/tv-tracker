import { ref, reactive } from 'vue'

export interface Toast {
  id: string
  type: 'success' | 'error' | 'warning' | 'info'
  title: string
  message?: string
  duration?: number
  persistent?: boolean
}

const toasts = ref<Toast[]>([])
let toastId = 0

export const useToast = () => {
  const addToast = (toast: Omit<Toast, 'id'>) => {
    const id = `toast-${++toastId}`
    const newToast: Toast = {
      id,
      duration: 4000,
      ...toast,
    }

    toasts.value.push(newToast)

    if (!newToast.persistent) {
      setTimeout(() => {
        removeToast(id)
      }, newToast.duration)
    }

    return id
  }

  const removeToast = (id: string) => {
    const index = toasts.value.findIndex(toast => toast.id === id)
    if (index > -1) {
      toasts.value.splice(index, 1)
    }
  }

  const clearAllToasts = () => {
    toasts.value = []
  }

  const success = (title: string, message?: string, options?: Partial<Toast>) => {
    return addToast({
      type: 'success',
      title,
      message,
      ...options,
    })
  }

  const error = (title: string, message?: string, options?: Partial<Toast>) => {
    return addToast({
      type: 'error',
      title,
      message,
      duration: 6000, // Longer duration for errors
      ...options,
    })
  }

  const warning = (title: string, message?: string, options?: Partial<Toast>) => {
    return addToast({
      type: 'warning',
      title,
      message,
      ...options,
    })
  }

  const info = (title: string, message?: string, options?: Partial<Toast>) => {
    return addToast({
      type: 'info',
      title,
      message,
      ...options,
    })
  }

  return {
    toasts,
    addToast,
    removeToast,
    clearAllToasts,
    success,
    error,
    warning,
    info,
  }
}