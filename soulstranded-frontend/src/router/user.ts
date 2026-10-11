import type { RouteRecordRaw } from 'vue-router'

const userRoutes: RouteRecordRaw[] = [
  {
    path: '',
    redirect: 'base-info',
  },
  {
    path: 'base-info',
    name: 'baseInfo',
    component: () => import('@/views/User/BaseInfo.vue'),
    meta: { layout: 'user', requiresAuth: true },
  },
  {
    path: 'statistics',
    name: 'statistics',
    component: () => import('@/views/User/Statistics.vue'),
    meta: { layout: 'user', requiresAuth: true },
  },
  {
    path: 'account-security',
    name: 'accountSecurity',
    component: () => import('@/views/User/AccountSecurity.vue'),
    meta: { layout: 'user', requiresAuth: true },
  },
]

export default userRoutes
