const MOVIES_CACHE_KEY = 'movies_cache';
const TV_SHOWS_CACHE_KEY = 'tvShows_cache';

// Cache checking and storing helper functions
const isCacheExpired = (timestamp: number) => {
  const CACHE_EXPIRATION_TIME = 15 * 60 * 1000; // 15 minutes in milliseconds
  return Date.now() - timestamp > CACHE_EXPIRATION_TIME;
};

const getFromCache = (cacheKey: string) => {
  const cachedData = localStorage.getItem(cacheKey);
  if (cachedData) {
    const { data, timestamp } = JSON.parse(cachedData);
    if (!isCacheExpired(timestamp)) {
      return data;
    }
  }
  return null;
};

const setToCache = (cacheKey: string, data: any) => {
  localStorage.setItem(cacheKey, JSON.stringify({
    data,
    timestamp: Date.now(),
  }));
};

export const fetchMovies = async (invokeFunction: any) => {
  // Check if movies are cached
  const cachedMovies = getFromCache(MOVIES_CACHE_KEY);
  if (cachedMovies) {
    return cachedMovies;
  }

  // Fetch from API if not cached
  try {
    const startTime = performance.now();
    const result: any[] = await invokeFunction('get_trending_movies');
    const movies = result.slice(0, 20).map((movie: any) => ({
      ...movie,
      poster_url: `https://image.tmdb.org/t/p/w500${movie.poster_path}`,
    }));

    // Cache the movies
    setToCache(MOVIES_CACHE_KEY, movies);
    const endTime = performance.now();
    return movies;
  } catch (error) {
    console.error('Failed to fetch trending movies:', error);
    return [];
  }
};

export const fetchTvShows = async (invokeFunction: any) => {
  // Check if TV shows are cached
  const cachedTvShows = getFromCache(TV_SHOWS_CACHE_KEY);
  if (cachedTvShows) {
    return cachedTvShows;
  }

  // Fetch from API if not cached
  try {
    const startTime = performance.now();
    const result: any[] = await invokeFunction('get_trending_tv');
    const tvShows = result.slice(0, 20).map((show: any) => ({
      ...show,
      poster_url: `https://image.tmdb.org/t/p/w500${show.poster_path}`,
    }));

    // Cache the TV shows
    setToCache(TV_SHOWS_CACHE_KEY, tvShows);
    const endTime = performance.now();
    return tvShows;
  } catch (error) {
    console.error('Failed to fetch trending TV shows:', error);
    return [];
  }
};

export const searchMovies = async (invokeFunction: any, query: string) => {
  try {
    const result: any[] = await invokeFunction('search_movies', { query });
    return result.slice(0, 15).map((movie: any) => ({
      ...movie,
      poster_url: `https://image.tmdb.org/t/p/w500${movie.poster_path}`,
    }));

  } catch (error) {
    console.error('Failed to search movies:', error);
    return [];
  }
}

export const searchShows = async (invokeFunction: any, query: string) => {
  try {
    const result: any[] = await invokeFunction('search_tv', { query });
    return result.slice(0, 15).map((show: any) => ({
      ...show,
      poster_url: `https://image.tmdb.org/t/p/w500${show.poster_path}`,
    }));

  } catch (error) {
    console.error('Failed to search shows:', error);
    return [];
  }
}

export const fetchMovieWatchlist = async (invokeFunction: any) => {
  // Fetch from API if not cached
  try {
    const result: any[] = await invokeFunction('get_watchlist_movies');
    const movies = result.map((movie: any) => ({
      ...movie,
      poster_url: `https://image.tmdb.org/t/p/w500${movie.poster_path}`,
    }));

    return movies;
  } catch (error) {
    console.error('Failed to fetch trending movies:', error);
    return [];
  }
}

export const fetchShowWatchlist = async (invokeFunction: any) => {

  try {
    const result: any[] = await invokeFunction('get_watchlist_shows');
    const shows = result.map((show: any) => ({
      ...show,
      poster_url: `https://image.tmdb.org/t/p/w500${show.poster_path}`,
    }));

    return shows;
  } catch (error) {
    console.error('Failed to fetch trending movies:', error);
    return [];
  }
}

export const getMovieDetails = async (invokeFunction: any, movieId: number) => {
  try {
    const result: any = await invokeFunction('get_movie_details', { id: movieId });
    return {
      ...result,
      poster_url: `https://image.tmdb.org/t/p/w500${result.poster_path}`,
    };
  } catch (error) {
    console.error('Failed to fetch movie details:', error);
    return null;
  }
}

export const getTvShowDetails = async (invokeFunction: any, showId: number) => {
  try {
    const result: any = await invokeFunction('get_tv_show_details', { id: showId });
    return {
      ...result,
      poster_url: `https://image.tmdb.org/t/p/w500${result.poster_path}`,
    };
  } catch (error) {
    console.error('Failed to fetch TV show details:', error);
    return null;
  }
}
