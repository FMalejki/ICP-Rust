import { createPinia } from 'pinia';
import { createApp } from 'vue';
import './index.scss';
import App from './App.vue';
import 'vue-router';
import router from './router/index.js';


createApp(App).use(router).use(createPinia()).mount('#app');
