import axios from 'axios'
import { ElMessage } from 'element-plus'
import { useRouter } from 'vue-router'
import { ERROR_CODE } from '@/constants/errorCode'
import { useUserStore } from '@/stores/user'

// 创建一个专用实例（可选，避免污染全局 axios）
const request = axios.create({
    baseURL: '/api',
    timeout: 15000
})

// 请求拦截器
request.interceptors.request.use((config) => {
    // 优先从 sessionStorage 读取 token（当前会话）
    let token = sessionStorage.getItem('token')
    // 如果 sessionStorage 没有，再从 localStorage 读取
    if (!token) {
        token = localStorage.getItem('token')
    }

    if (token) {
        config.headers.Authorization = `Bearer ${token}`
    }
    return config
}, (error) => {
    ElMessage.error('请求失败')
    return Promise.reject(error)
})

// 响应拦截器
request.interceptors.response.use(
    // 成功响应
    (response) => {
        const { code, message, data } = response.data

        if (code === ERROR_CODE.SUCCESS) {
            return data
        }

        switch (code) {
            case ERROR_CODE.VALIDATION_FAILED:
                ElMessage.error('参数校验失败')
                break
            case ERROR_CODE.CAPTCHA_ERROR:
                ElMessage.error('验证码错误')
                break
            case ERROR_CODE.EMAIL_EXISTS:
                ElMessage.error('该邮箱已被注册')
                break
            case ERROR_CODE.USER_NOT_FOUND:
                ElMessage.error('用户不存在')
                break
            case ERROR_CODE.PASSWORD_ERROR:
                ElMessage.error('用户名或密码错误')
                break
            case ERROR_CODE.TOKEN_INVALID:
            case ERROR_CODE.UNAUTHORIZED:
                ElMessage.error('登录已过期，请重新登录')
                // 使用 userStore 清除所有存储
                const userStore = useUserStore()
                userStore.logout()
                const router = useRouter()
                router.push('/auth/login')
                break
            case ERROR_CODE.FORBIDDEN:
                ElMessage.error('权限不足，无法访问')
                break
            case ERROR_CODE.SYSTEM_ERROR:
                ElMessage.error('系统繁忙，请稍后再试')
                break
            default:
                ElMessage.error(message || '操作失败')
        }

        return Promise.reject(new Error(message))
    },
    (error) => {
        if (error.code === 'ECONNABORTED' && error.message.includes('timeout')) {
            ElMessage.error('请求超时，请检查网络')
        } else if (error.response) {
            const status = error.response.status
            if (status >= 500) {
                ElMessage.error('服务暂时不可用')
            } else if (status === 404) {
                ElMessage.error('请求的资源不存在')
            } else {
                ElMessage.error('请求失败，请重试')
            }
        } else {
            ElMessage.error('网络连接失败，请检查网络')
        }

        return Promise.reject(error)
    }
)

export default request
