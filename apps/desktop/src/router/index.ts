import { createRouter, createWebHistory } from 'vue-router';
import HomePage from '../views/HomePage.vue';
import PopularPage from '../views/PopularPage.vue';
import WatchlistPage from '../views/WatchlistPage.vue';
import MovieDetailsPage from '../views/MovieDetailsPage.vue';
import TvShowDetailsPage from '../views/TvShowDetailsPage.vue';

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
  }
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
