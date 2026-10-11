<template>
    <div class="login-container">
        <!-- 右侧登录表单区 -->
        <div class="login-form-container">
            <div class="login-form-wrapper">
                <div class="login-logo">
                    <Logo />
                </div>

                <div class="decoration-content">
                    <p>这个世界，总有人懂你</p>
                </div>

                <el-form ref="loginFormRef" :model="loginForm" :rules="loginRules" class="login-form">
                    <el-form-item prop="email">
                        <el-input v-model="loginForm.email" placeholder="请输入邮箱" clearable size="large">
                            <template #prefix>
                                <el-icon>
                                    <Message />
                                </el-icon>
                            </template>
                        </el-input>
                    </el-form-item>

                    <el-form-item prop="password">
                        <el-input v-model="loginForm.password" type="password" placeholder="请输入密码" show-password
                            clearable size="large" @keyup.enter="handleLogin">
                            <template #prefix>
                                <el-icon>
                                    <Lock />
                                </el-icon>
                            </template>
                        </el-input>
                    </el-form-item>

                    <div class="login-options">
                        <el-checkbox v-model="loginForm.rememberMe">记住我</el-checkbox>
                        <el-link type="primary" :underline="false" @click="handleForgotPassword">
                            忘记密码？
                        </el-link>
                    </div>

                    <el-form-item>
                        <el-button type="primary" :loading="isLoggingIn" @click="handleLogin" size="large"
                            class="login-button" block>
                            登录
                        </el-button>
                    </el-form-item>
                </el-form>

                <!-- 注册入口 -->
                <div class="register-entry">
                    <span>还没有账号？</span>
                    <el-link type="primary" :underline="false" @click="handleRegister">
                        立即注册
                    </el-link>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue'
import { ElMessage } from 'element-plus'
import type { FormInstance, FormRules } from 'element-plus'
import { useRouter } from 'vue-router'
import Logo from '@/components/Logo.vue'
import { Message, Lock } from '@element-plus/icons-vue'
import { authApi } from '@/api'
import { useUserStore } from '@/stores/user'
import type { LoginForm } from '@/types/auth'

// 路由
const router = useRouter()

// 使用用户 store
const userStore = useUserStore()

// 表单引用
const loginFormRef = ref<FormInstance | null>(null)

// 登录加载状态
const isLoggingIn = ref(false)

// 登录表单数据
const loginForm = reactive<LoginForm>({
    email: '',
    password: '',
    rememberMe: false
})

// 表单验证规则
const loginRules = reactive<FormRules<LoginForm>>({
    email: [
        { required: true, message: '请输入邮箱', trigger: 'blur' },
        { type: 'email', message: '请输入正确的邮箱格式', trigger: 'blur' }
    ],
    password: [
        { required: true, message: '请输入密码', trigger: 'blur' },
        { min: 6, max: 30, message: '密码长度在 6 到 30 个字符', trigger: 'blur' }
    ]
})

// 处理登录
const handleLogin = async () => {
    if (!loginFormRef.value) return

    try {
        // 表单验证
        await loginFormRef.value.validate()

        // 调用登录 API
        isLoggingIn.value = true
        const responseData = await authApi.login(loginForm)

        // 使用 userStore 保存用户信息和 token
        userStore.login(responseData, loginForm.rememberMe)

        ElMessage.success('登录成功')
        router.push('/post')

    } catch (error: any) {
    } finally {
        isLoggingIn.value = false
    }
}

// 处理注册
const handleRegister = () => {
    router.push('/auth/register')
}

const handleForgotPassword = () => {
    router.push('/auth/forget-password')
}
</script>

<style scoped>
.login-container {
    display: flex;
    min-height: 100vh;
}

/* 装饰文本 */
.decoration-content {
    text-align: center;
    margin-bottom: 2rem;
}

/* 右侧登录表单区 */
.login-form-container {
    flex: 0 0 420px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: white;
    box-shadow: -5px 0 25px rgba(0, 0, 0, 0.1);
    animation: fadeInUp 0.5s ease-out;
    margin: auto 0;
    /* 上下居中 */
    height: auto;
    /* 自适应子元素高度 */
    padding: 1rem;
    /* 添加适当内边距 */
    border-radius: 20px;
    /* 圆角效果 */
}

@keyframes fadeInUp {
    from {
        transform: translateY(30px);
        opacity: 0;
    }

    to {
        transform: translateY(0);
        opacity: 1;
    }
}

.login-form-wrapper {
    width: 100%;
    max-width: 360px;
    padding: 2rem;
}

/* Logo */
.login-logo {
    display: flex;
    flex-direction: column;
    align-items: center;
    margin-bottom: 2rem;
}

/* 装饰文字 */
.decoration-content {
    text-align: center;
    margin-bottom: 2.5rem;
}

.decoration-content p {
    font-size: 1.25rem;
    font-weight: 500;
    color: #333;
    line-height: 1.6;
    letter-spacing: 0.02em;
    font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
}

/* 登录表单选项 */
.login-options {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1.5rem;
}

.login-button {
    height: 48px;
    width: 100%;
    font-size: 1rem;
    font-weight: 600;
    background-color: #1660ab;
    border: none;
    border-radius: 8px;
    transition: all 0.3s ease;
    margin-bottom: 1rem;
}

.login-button:hover {
    transform: translateY(-1px);
    box-shadow: 0 5px 15px rgba(22, 96, 171, 0.4);
}

/* 注册入口 */
.register-entry {
    text-align: center;
    color: #666;
    font-size: 0.9rem;
    margin-top: 2rem;
    padding-top: 1.5rem;
    border-top: 1px solid #f0f0f0;
}

.register-entry .el-link {
    font-weight: 600;
    margin-left: 0.5rem;
    font-size: 1rem;
}

/* 响应式设计 */
@media (max-width: 768px) {
    .login-container {
        flex-direction: column;
    }

    .login-form-container {
        flex: 1;
        width: 100%;
        box-shadow: none;
    }

    .login-form-wrapper {
        max-width: 100%;
        padding: 1.5rem;
    }
}
</style>