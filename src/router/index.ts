import { createRouter, createWebHistory } from 'vue-router'
import LoginRegister from '../components/LoginRegister.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'login',
      component: LoginRegister,
    },
  ],
})

export default router
