// router/index.js
import { createRouter, createWebHistory } from 'vue-router';
import Home from '../views/home.vue'; // Zaimportuj komponenty widoku, które chcesz użyć w trasach
import Login from '../views/login.vue'; 

const routes = [
  {
    path: '/',
    name: 'Home',
    component: Home // Użyj komponentu dla strony głównej
  },
  {
    path: '/login',
    name: 'Login',
    component: Login
  }
  // Dodaj inne trasy według potrzeb
]

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes
})

export default router