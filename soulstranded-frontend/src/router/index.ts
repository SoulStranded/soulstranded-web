import { createRouter, createWebHistory } from 'vue-router'
import authRoutes from './auth'
import postRoutes from './post'
import userRoutes from './user'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      redirect: '/post',
    },
    {
      path: '/post',
      children: postRoutes,
    },
    {
      path: '/auth',
      children: authRoutes,
    },
    {
      path: '/user',
      children: userRoutes,
    },
    // 重定向到首页
    { path: '/:pathMatch(.*)*', redirect: '/post' },
  ],
})

// 全局前置守卫
router.beforeEach((to, from, next) => {
  // 检查路由是否需要登录
  if (to.meta.requiresAuth) {
    // 这里需要从 store 获取登录状态
    // 由于在路由守卫中不能直接使用 store，我们通过 localStorage 检查
    const token = sessionStorage.getItem('token') || localStorage.getItem('token')
    if (!token) {
      // 未登录，跳转到登录页，并保存目标路由
      next({
        path: '/auth/login',
        query: { redirect: to.fullPath }
      })
    } else {
      next()
    }
  } else {
    next()
  }
})

export default router
