import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { LoginResponse } from '@/types'

export const useUserStore = defineStore('user', () => {
    // 状态
    const token = ref<string>('')
    const user = ref<LoginResponse['user'] | null>(null)
    const isLoggedIn = ref(false)

    // 登录 - 保存用户信息和 token
    const login = (userData: LoginResponse, rememberMe: boolean = false) => {
        token.value = userData.token
        user.value = userData.user
        isLoggedIn.value = true

        // 计算 token 过期时间（毫秒）
        const expirationTime = rememberMe ? 30 * 24 * 60 * 60 * 1000 : 24 * 60 * 60 * 1000
        const expiresAt = Date.now() + expirationTime

        // 根据记住我选项选择存储方式
        if (rememberMe) {
            // 长期存储（30天）
            localStorage.setItem('token', userData.token)
            localStorage.setItem('user', JSON.stringify(userData.user))
            localStorage.setItem('tokenExpiresAt', expiresAt.toString())
        } else {
            // 会话存储（24小时）
            sessionStorage.setItem('token', userData.token)
            sessionStorage.setItem('user', JSON.stringify(userData.user))
            sessionStorage.setItem('tokenExpiresAt', expiresAt.toString())
        }
    }

    // 登出 - 清除用户信息和 token
    const logout = () => {
        token.value = ''
        user.value = null
        isLoggedIn.value = false

        // 清除所有存储
        localStorage.removeItem('token')
        localStorage.removeItem('user')
        localStorage.removeItem('tokenExpiresAt')
        sessionStorage.removeItem('token')
        sessionStorage.removeItem('user')
        sessionStorage.removeItem('tokenExpiresAt')
    }

    // 初始化 - 从存储恢复用户状态
    const initialize = () => {
        // 优先从 sessionStorage 读取（当前会话）
        let savedToken = sessionStorage.getItem('token')
        let savedUser = sessionStorage.getItem('user')
        let savedExpiresAt = sessionStorage.getItem('tokenExpiresAt')

        // 如果 sessionStorage 没有，再从 localStorage 读取
        if (!savedToken) {
            savedToken = localStorage.getItem('token')
            savedUser = localStorage.getItem('user')
            savedExpiresAt = localStorage.getItem('tokenExpiresAt')
        }

        if (savedToken && savedUser && savedExpiresAt) {
            try {
                // 检查 token 是否过期
                const expiresAt = parseInt(savedExpiresAt, 10)
                const now = Date.now()

                if (now >= expiresAt) {
                    // Token 已过期，清除存储
                    console.log('Token has expired, clearing storage')
                    logout()
                    return
                }

                // Token 未过期，恢复用户状态
                token.value = savedToken
                user.value = JSON.parse(savedUser)
                isLoggedIn.value = true
            } catch (error) {
                console.error('Failed to parse user data from storage:', error)
                logout()
            }
        }
    }

    return {
        // 状态
        token,
        user,
        isLoggedIn,

        // 方法
        login,
        logout,
        initialize
    }
})
