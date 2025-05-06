import {createRouter, createWebHistory, RouteRecordRaw} from 'vue-router';

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    component: ()=>import('./pages/index.vue')
  },
  {
    path: '/dialog',
    component: ()=>import('./pages/dialog.vue')
  }
]

export default createRouter({
  routes,
  history: createWebHistory()
})