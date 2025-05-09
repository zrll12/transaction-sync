import { createRouter, createWebHistory } from 'vue-router'

export default createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      component: () => import('./pages/index.vue')
    },
    {
      path: '/dialog',
      component: () => import('./pages/dialog.vue')
    },
    {
      path: '/preview',
      component: () => import('./pages/preview.vue')
    }
  ]
})