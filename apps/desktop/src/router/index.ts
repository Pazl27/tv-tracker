import RatingTest from '../components/RatingTest.vue';
import HomePage from '../views/HomePage.vue';
import MovieDetailsPage from '../views/MovieDetailsPage.vue';
import PopularPage from '../views/PopularPage.vue';
import TvShowDetailsPage from '../views/TvShowDetailsPage.vue';
import WatchedPage from '../views/WatchedPage.vue';
import WatchlistPage from '../views/WatchlistPage.vue';
import { createRouter, createWebHistory } from 'vue-router';

const routes = [
  {
    path: '/',
    name: 'Home',
    component: HomePage,
  },
  {
    path: '/popular',
    name: 'Popular',
    component: PopularPage,
  },
  {
    path: '/watchlist',
    name: 'Watchlist',
    component: WatchlistPage,
  },
  {
    path: '/movie/:id',
    name: 'MovieDetails',
    component: MovieDetailsPage,
    props: true
  },
  {
    path: '/tv/:id',
    name: 'TvShowDetails',
    component: TvShowDetailsPage,
    props: true
  },
  {
    path: '/watched',
    name: 'Watched',
    component: WatchedPage
  },
  {
    path: '/test-rating',
    name: 'RatingTest',
    component: RatingTest
  }
];

const router = createRouter({
	history: createWebHistory(),
	routes
});

export default router;
