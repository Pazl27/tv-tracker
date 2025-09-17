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
        poster_url: `https://image.tmdb.org/t/p/original${movie.poster_path}`,
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
      const result: any[] = await invoke('get_watchlist_shows')
      watchlistTvShows.value = result.map((show: any) => ({
        ...show,
        poster_url: `https://image.tmdb.org/t/p/original${show.poster_path}`,
      }))
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
          poster_url: `https://image.tmdb.org/t/p/original${movie.poster_path}`,
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
      await invoke('add_show_to_watchlist', { show })
      
      // Add to local state if not already present
      if (!isTvShowInWatchlist(show.id)) {
        const showWithPosterUrl = {
          ...show,
          poster_url: `https://image.tmdb.org/t/p/original${show.poster_path}`,
        }
        watchlistTvShows.value.push(showWithPosterUrl)
      }
      
      return true
    } catch (error) {
      console.error('Failed to add TV show to watchlist:', error)
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
      await invoke('remove_show_from_watchlist', { show })
      
      // Remove from local state
      const index = watchlistTvShows.value.findIndex(s => s.id === show.id)
      if (index > -1) {
        watchlistTvShows.value.splice(index, 1)
      }
      
      return true
    } catch (error) {
      console.error('Failed to remove TV show from watchlist:', error)
      throw error
    }
  }

  // Clear watchlist data
  const clearWatchlistData = () => {
    watchlistMovies.value = []
    watchlistTvShows.value = []
  }

  // Initialize watchlist data
  const initializeWatchlist = async () => {
    try {
      await Promise.all([
        loadWatchlistMovies(),
        loadWatchlistTvShows()
      ])
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