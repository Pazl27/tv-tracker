const MOVIES_CACHE_KEY = 'movies_cache';
const TV_SHOWS_CACHE_KEY = 'tvShows_cache';

// Cache checking and storing helper functions
const isCacheExpired = (timestamp: number) => {
  const CACHE_EXPIRATION_TIME = 60 * 60 * 1000; // 1 hour in milliseconds
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
    console.log('Using cached movies');
    return cachedMovies;
  }

  // Fetch from API if not cached
  try {
    const result: any[] = await invokeFunction('get_trending_movies');
    const movies = result.map((movie: any) => ({
      ...movie,
      poster_url: `https://image.tmdb.org/t/p/original${movie.poster_path}`,
    }));

    // Cache the movies
    setToCache(MOVIES_CACHE_KEY, movies);
    console.log('Fetched and cached movies');
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
    console.log('Using cached TV shows');
    return cachedTvShows;
  }

  // Fetch from API if not cached
  try {
    const result: any[] = await invokeFunction('get_trending_tv');
    const tvShows = result.map((show: any) => ({
      ...show,
      poster_url: `https://image.tmdb.org/t/p/original${show.poster_path}`,
    }));
    console.log(tvShows.length)

    // Cache the TV shows
    setToCache(TV_SHOWS_CACHE_KEY, tvShows);
    console.log('Fetched and cached TV shows');
    return tvShows;
  } catch (error) {
    console.error('Failed to fetch trending TV shows:', error);
    return [];
  }
};

export const searchMovies = async (invokeFunction: any, query: string) => {
  try {
    const result: any[] = await invokeFunction('search_movies', { query });
    return result.map((movie: any) => ({
      ...movie,
      poster_url: `https://image.tmdb.org/t/p/original${movie.poster_path}`,
    }));

  } catch (error) {
    console.error('Failed to search movies:', error);
    return [];
  }
}

export const searchShows = async (invokeFunction: any, query: string) => {
  try {
    const result: any[] = await invokeFunction('search_tv', { query });
    return result.map((show: any) => ({
      ...show,
      poster_url: `https://image.tmdb.org/t/p/original${show.poster_path}`,
    }));

  } catch (error) {
    console.error('Failed to search movies:', error);
    return [];
  }
}

export const fetchMovieWatchlist = async (invokeFunction: any) => {
  // Fetch from API if not cached
  try {
    const result: any[] = await invokeFunction('get_watchlist_movies');
    const movies = result.map((movie: any) => ({
      ...movie,
      poster_url: `https://image.tmdb.org/t/p/original${movie.poster_path}`,
    }));

    return movies;
  } catch (error) {
    console.error('Failed to fetch trending movies:', error);
    return [];
  }

}
