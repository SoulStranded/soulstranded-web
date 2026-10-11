import type { RouteRecordRaw } from 'vue-router'

const postRoutes: RouteRecordRaw[] = [
  {
    path: '',
    name: 'home',
    component: () => import('@/views/Post/Home.vue'),
    meta: { layout: 'post' },
  },
  {
    path: 'publish',
    name: 'publish',
    component: () => import('@/views/Post/PublishPost.vue'),
    meta: { layout: 'post', requiresAuth: true },
  },
]

export default postRoutes
