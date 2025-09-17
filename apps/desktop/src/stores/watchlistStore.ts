import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'

interface Movie {
  id: number
  title: string
  poster_path: string
  poster_url: string
  release_date: string
  vote_average: number
  overview: string
  runtime?: number
  genres?: string[]
  vote_count?: number
  popularity?: number
}

interface TvShow {
  id: number
  name: string
  poster_path: string
  poster_url: string
  first_air_date: string
  vote_average: number
  overview: string
}

const watchlistMovies = ref<Movie[]>([])
const watchlistTvShows = ref<TvShow[]>([])
const isLoadingMovies = ref(false)
const isLoadingTvShows = ref(false)

export const useWatchlistStore = () => {
  // Computed properties
  const watchlistMovieIds = computed(() => 
    new Set(watchlistMovies.value.map(movie => movie.id))
  )
  
  const watchlistTvShowIds = computed(() => 
    new Set(watchlistTvShows.value.map(show => show.id))
  )

  const isMovieInWatchlist = (movieId: number) => {
    return watchlistMovieIds.value.has(movieId)
  }

  const isTvShowInWatchlist = (showId: number) => {
    return watchlistTvShowIds.value.has(showId)
  }

  // Load watchlist data
  const loadWatchlistMovies = async () => {
    if (isLoadingMovies.value) return
    
    isLoadingMovies.value = true
    try {
      const result: any[] = await invoke('get_watchlist_movies')
      watchlistMovies.value = result.map((movie: any) => ({
        ...movie,
        poster_url: `https://image.tmdb.org/t/p/w500${movie.poster_path}`,
      }))
    } catch (error) {
      console.error('Failed to load watchlist movies:', error)
      throw error
    } finally {
      isLoadingMovies.value = false
    }
  }

  const loadWatchlistTvShows = async () => {
    if (isLoadingTvShows.value) return
    
    isLoadingTvShows.value = true
    try {
      console.log('Loading watchlist TV shows...')
      const result: any[] = await invoke('get_watchlist_shows')
      console.log('Raw TV shows from backend:', result)
      
      watchlistTvShows.value = result.map((show: any) => ({
        ...show,
        poster_url: `https://image.tmdb.org/t/p/w500${show.poster_path}`,
      }))
      
      console.log('Processed TV shows for store:', watchlistTvShows.value)
      console.log('Total TV shows in watchlist:', watchlistTvShows.value.length)
    } catch (error) {
      console.error('Failed to load watchlist TV shows:', error)
      throw error
    } finally {
      isLoadingTvShows.value = false
    }
  }

  // Add items to watchlist
  const addMovieToWatchlist = async (movie: Movie) => {
    try {
      await invoke('add_movie_to_watchlist', { movie })
      
      // Add to local state if not already present
      if (!isMovieInWatchlist(movie.id)) {
        const movieWithPosterUrl = {
          ...movie,
          poster_url: `https://image.tmdb.org/t/p/w500${movie.poster_path}`,
        }
        watchlistMovies.value.push(movieWithPosterUrl)
      }
      
      return true
    } catch (error) {
      console.error('Failed to add movie to watchlist:', error)
      throw error
    }
  }

  const addTvShowToWatchlist = async (show: TvShow) => {
    try {
      console.log('Adding TV show to watchlist:', show)
      await invoke('add_show_to_watchlist', { show })
      console.log('Successfully added TV show to backend')
      
      // Add to local state if not already present
      if (!isTvShowInWatchlist(show.id)) {
        const showWithPosterUrl = {
          ...show,
          poster_url: `https://image.tmdb.org/t/p/w500${show.poster_path}`,
        }
        watchlistTvShows.value.push(showWithPosterUrl)
        console.log('Added TV show to local state. Total shows:', watchlistTvShows.value.length)
      } else {
        console.log('TV show already in watchlist')
      }
      
      return true
    } catch (error) {
      console.error('Failed to add TV show to watchlist:', error)
      console.error('Show data:', show)
      throw error
    }
  }

  // Remove items from watchlist
  const removeMovieFromWatchlist = async (movie: Movie) => {
    try {
      await invoke('remove_movie_from_watchlist', { movie })
      
      // Remove from local state
      const index = watchlistMovies.value.findIndex(m => m.id === movie.id)
      if (index > -1) {
        watchlistMovies.value.splice(index, 1)
      }
      
      return true
    } catch (error) {
      console.error('Failed to remove movie from watchlist:', error)
      throw error
    }
  }

  const removeTvShowFromWatchlist = async (show: TvShow) => {
    try {
      console.log('Removing TV show from watchlist:', show)
      await invoke('remove_show_from_watchlist', { show })
      console.log('Successfully removed TV show from backend')
      
      // Remove from local state
      const index = watchlistTvShows.value.findIndex(s => s.id === show.id)
      if (index > -1) {
        watchlistTvShows.value.splice(index, 1)
        console.log('Removed TV show from local state. Remaining shows:', watchlistTvShows.value.length)
      } else {
        console.log('TV show not found in local state')
      }
      
      return true
    } catch (error) {
      console.error('Failed to remove TV show from watchlist:', error)
      console.error('Show data:', show)
      throw error
    }
  }

  // Clear watchlist data
  const clearWatchlistData = () => {
    watchlistMovies.value = []
    watchlistTvShows.value = []
  }

  // Initialize watchlist data progressively
  const initializeWatchlist = async () => {
    try {
      // Load movies first (more commonly accessed)
      await loadWatchlistMovies()
      
      // Load TV shows after a small delay to prevent blocking
      setTimeout(() => {
        loadWatchlistTvShows()
      }, 100)
    } catch (error) {
      console.error('Failed to initialize watchlist:', error)
    }
  }

  return {
    // State
    watchlistMovies,
    watchlistTvShows,
    isLoadingMovies,
    isLoadingTvShows,
    
    // Computed
    watchlistMovieIds,
    watchlistTvShowIds,
    isMovieInWatchlist,
    isTvShowInWatchlist,
    
    // Actions
    loadWatchlistMovies,
    loadWatchlistTvShows,
    addMovieToWatchlist,
    addTvShowToWatchlist,
    removeMovieFromWatchlist,
    removeTvShowFromWatchlist,
    clearWatchlistData,
    initializeWatchlist,
  }
}