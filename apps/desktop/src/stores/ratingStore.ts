import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'

interface RatedMovie {
  id: number
  title: string
  poster_path: string
  rating: number
}

interface RatedTvShow {
  id: number
  name: string
  poster_path: string
  first_air_date: string
  vote_average: number
  overview: string
  rating: number
}

const movieRatings = ref<Map<number, number>>(new Map())
const tvShowRatings = ref<Map<number, number>>(new Map())
const ratedMovies = ref<RatedMovie[]>([])
const ratedTvShows = ref<RatedTvShow[]>([])
const isLoadingMovieRatings = ref(false)
const isLoadingTvShowRatings = ref(false)

export const useRatingStore = () => {
  // Computed properties
  const getMovieRating = computed(() => (movieId: number) => {
    return movieRatings.value.get(movieId) || 0
  })

  const getTvShowRating = computed(() => (showId: number) => {
    return tvShowRatings.value.get(showId) || 0
  })

  const isMovieRated = computed(() => (movieId: number) => {
    return movieRatings.value.has(movieId)
  })

  const isTvShowRated = computed(() => (showId: number) => {
    return tvShowRatings.value.has(showId)
  })

  // Load ratings
  const loadMovieRating = async (movieId: number) => {
    try {
      const rating: number | null = await invoke('get_movie_rating', { movieId })
      if (rating !== null) {
        movieRatings.value.set(movieId, rating)
      } else {
        movieRatings.value.delete(movieId)
      }
      return rating
    } catch (error) {
      console.error('Failed to load movie rating:', error)
      return null
    }
  }

  const loadTvShowRating = async (showId: number) => {
    try {
      const rating: number | null = await invoke('get_tv_show_rating', { showId })
      if (rating !== null) {
        tvShowRatings.value.set(showId, rating)
      } else {
        tvShowRatings.value.delete(showId)
      }
      return rating
    } catch (error) {
      console.error('Failed to load TV show rating:', error)
      return null
    }
  }

  const loadAllRatedMovies = async () => {
    if (isLoadingMovieRatings.value) return

    isLoadingMovieRatings.value = true
    try {
      const result: RatedMovie[] = await invoke('get_all_rated_movies')
      ratedMovies.value = result

      // Update the ratings map
      movieRatings.value.clear()
      result.forEach(movie => {
        movieRatings.value.set(movie.id, movie.rating)
      })
    } catch (error) {
      console.error('Failed to load rated movies:', error)
      throw error
    } finally {
      isLoadingMovieRatings.value = false
    }
  }

  const loadAllRatedTvShows = async () => {
    if (isLoadingTvShowRatings.value) return

    isLoadingTvShowRatings.value = true
    try {
      const result: RatedTvShow[] = await invoke('get_all_rated_tv_shows')
      ratedTvShows.value = result

      // Update the ratings map
      tvShowRatings.value.clear()
      result.forEach(show => {
        tvShowRatings.value.set(show.id, show.rating)
      })
    } catch (error) {
      console.error('Failed to load rated TV shows:', error)
      throw error
    } finally {
      isLoadingTvShowRatings.value = false
    }
  }

  // Rate items
  const rateMovie = async (movie: any, rating: number) => {
    try {
      // Validate rating
      if (rating < 0.5 || rating > 5.0 || (rating * 2) % 1 !== 0) {
        throw new Error('Rating must be between 0.5 and 5.0 in 0.5 increments')
      }

      await invoke('rate_movie', { movie, rating })

      // Update local state
      movieRatings.value.set(movie.id, rating)

      // Update rated movies list
      const existingIndex = ratedMovies.value.findIndex(m => m.id === movie.id)
      const ratedMovie: RatedMovie = {
        id: movie.id,
        title: movie.title,
        poster_path: movie.poster_path,
        rating
      }

      if (existingIndex >= 0) {
        ratedMovies.value[existingIndex] = ratedMovie
      } else {
        ratedMovies.value.push(ratedMovie)
      }

      return true
    } catch (error) {
      console.error('Failed to rate movie:', error)
      throw error
    }
  }

  const rateTvShow = async (show: any, rating: number) => {
    try {
      // Validate rating
      if (rating < 0.5 || rating > 5.0 || (rating * 2) % 1 !== 0) {
        throw new Error('Rating must be between 0.5 and 5.0 in 0.5 increments')
      }

      await invoke('rate_tv_show', { show, rating })

      // Update local state
      tvShowRatings.value.set(show.id, rating)

      // Update rated TV shows list
      const existingIndex = ratedTvShows.value.findIndex(s => s.id === show.id)
      const ratedTvShow: RatedTvShow = {
        id: show.id,
        name: show.name,
        poster_path: show.poster_path,
        first_air_date: show.first_air_date || '',
        vote_average: show.vote_average || 0,
        overview: show.overview || '',
        rating
      }

      if (existingIndex >= 0) {
        ratedTvShows.value[existingIndex] = ratedTvShow
      } else {
        ratedTvShows.value.push(ratedTvShow)
      }

      return true
    } catch (error) {
      console.error('Failed to rate TV show:', error)
      throw error
    }
  }

  // Remove ratings
  const removeMovieRating = async (movieId: number) => {
    try {
      await invoke('remove_movie_rating', { movieId })

      // Update local state
      movieRatings.value.delete(movieId)

      // Remove from rated movies list
      const index = ratedMovies.value.findIndex(m => m.id === movieId)
      if (index >= 0) {
        ratedMovies.value.splice(index, 1)
      }

      return true
    } catch (error) {
      console.error('Failed to remove movie rating:', error)
      throw error
    }
  }

  const removeTvShowRating = async (showId: number) => {
    try {
      await invoke('remove_tv_show_rating', { showId })

      // Update local state
      tvShowRatings.value.delete(showId)

      // Remove from rated TV shows list
      const index = ratedTvShows.value.findIndex(s => s.id === showId)
      if (index >= 0) {
        ratedTvShows.value.splice(index, 1)
      }

      return true
    } catch (error) {
      console.error('Failed to remove TV show rating:', error)
      throw error
    }
  }

  // Clear all ratings data
  const clearRatingsData = () => {
    movieRatings.value.clear()
    tvShowRatings.value.clear()
    ratedMovies.value = []
    ratedTvShows.value = []
  }

  // Initialize ratings data
  const initializeRatings = async () => {
    try {
      // Load both movie and TV show ratings
      await Promise.all([
        loadAllRatedMovies(),
        loadAllRatedTvShows()
      ])
    } catch (error) {
      console.error('Failed to initialize ratings:', error)
    }
  }

  return {
    // State
    movieRatings,
    tvShowRatings,
    ratedMovies,
    ratedTvShows,
    isLoadingMovieRatings,
    isLoadingTvShowRatings,

    // Computed
    getMovieRating,
    getTvShowRating,
    isMovieRated,
    isTvShowRated,

    // Actions
    loadMovieRating,
    loadTvShowRating,
    loadAllRatedMovies,
    loadAllRatedTvShows,
    rateMovie,
    rateTvShow,
    removeMovieRating,
    removeTvShowRating,
    clearRatingsData,
    initializeRatings,
  }
}
