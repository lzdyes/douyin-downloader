import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: '/', component: () => import('@/components/multiple/index.vue') },
    { path: '/single', component: () => import('@/components/single/index.vue') },
    { path: '/about', component: () => import('@/components/about/index.vue') },
  ],
})

export default router
