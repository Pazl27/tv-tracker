import { createRouter, createWebHistory } from 'vue-router';
import HomePage from '../views/HomePage.vue';
import PopularPage from '../views/PopularPage.vue';
import WatchlistPage from '../views/WatchlistPage.vue';

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
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
