import type { RouteRecordRaw } from 'vue-router'

const authRoutes: RouteRecordRaw[] = [
    {
        path: 'login',
        name: 'login',
        component: () => import('@/views/Auth/Login.vue'),
        meta: { layout: 'auth' },
    },
    {
        path: 'register',
        name: 'register',
        component: () => import('@/views/Auth/Register.vue'),
        meta: { layout: 'auth' },
    },
    {
        path: 'forget-password',
        name: 'forgetPassword',
        component: () => import('@/views/Auth/ForgetPassword.vue'),
        meta: { layout: 'auth' },
    },
]

export default authRoutes
