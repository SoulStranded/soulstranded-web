<template>
  <div class="register-container">

    <!-- 右侧注册表单区 -->
    <div class="register-form-container">
      <div class="register-form-wrapper">
        <!-- Logo -->
        <div class="register-logo">
          <Logo />
        </div>

        <div class="decoration-content">
          <p>开启你的心灵之旅</p>
        </div>

        <!-- 注册表单 -->
        <el-form ref="registerFormRef" :model="registerForm" :rules="registerRules" class="register-form">

          <el-form-item prop="username">
            <el-input v-model="registerForm.username" placeholder="请输入用户名" clearable size="large">
              <template #prefix>
                <el-icon>
                  <User />
                </el-icon>
              </template>
            </el-input>
          </el-form-item>

          <el-form-item prop="email">
            <el-input v-model="registerForm.email" type="email" placeholder="请输入邮箱" clearable size="large">
              <template #prefix>
                <el-icon>
                  <Message />
                </el-icon>
              </template>
            </el-input>
          </el-form-item>

          <el-form-item prop="captcha">
            <div class="captcha-container">
              <el-input v-model="registerForm.captcha" placeholder="请输入验证码" clearable size="large">
                <template #prefix>
                  <el-icon>
                    <Lock />
                  </el-icon>
                </template>
              </el-input>
              <el-button type="primary" @click="sendCaptcha" :disabled="captchaCountdown > 0 || isCaptchaSending"
                size="large" class="captcha-button">
                {{ captchaCountdown > 0 ? `${captchaCountdown}秒后重试` : '立即发送' }}
              </el-button>
            </div>
          </el-form-item>

          <el-form-item prop="password">
            <el-input v-model="registerForm.password" type="password" placeholder="请输入密码" show-password clearable
              size="large">
              <template #prefix>
                <el-icon>
                  <Lock />
                </el-icon>
              </template>
            </el-input>
          </el-form-item>

          <el-form-item prop="confirmPassword">
            <el-input v-model="registerForm.confirmPassword" type="password" placeholder="请确认密码" show-password clearable
              size="large" @keyup.enter="handleRegister">
              <template #prefix>
                <el-icon>
                  <Lock />
                </el-icon>
              </template>
            </el-input>
          </el-form-item>

          <el-form-item>
            <el-button type="primary" :loading="isRegistering" @click="handleRegister" size="large"
              class="register-button" block>
              注册
            </el-button>
          </el-form-item>
        </el-form>

        <!-- 登录入口 -->
        <div class="login-entry">
          <span>已有账号？</span>
          <el-link type="primary" :underline="false" @click="handleLogin">
            立即登录
          </el-link>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue'
import type { FormInstance, FormRules } from 'element-plus'
import Logo from '@/components/Logo.vue'
import { User, Message, Lock } from '@element-plus/icons-vue'
import { authApi } from '@/api'
import { useAuthForm } from '@/composables/useAuthForm'
import type { RegisterForm } from '@/types/auth'

// 表单引用
const registerFormRef = ref<FormInstance | null>(null)

// 注册表单数据
const registerForm = reactive<RegisterForm>({
  username: '',
  email: '',
  password: '',
  confirmPassword: '',
  captcha: ''
})

// 扩展验证规则，添加 username 字段
const registerRules = reactive<FormRules<RegisterForm>>({
  username: [
    { required: true, message: '请输入用户名', trigger: 'blur' },
    { min: 3, max: 20, message: '用户名长度在 3 到 20 个字符', trigger: 'blur' }
  ]
})

// 使用 useAuthForm composable
const {
  isSubmitting: isRegistering,
  isCaptchaSending,
  captchaCountdown,
  commonRules,
  sendCaptcha,
  handleSubmit: handleRegister,
  handleLogin
} = useAuthForm(
  registerForm,
  registerFormRef,
  (data) => authApi.register(data).then(() => { }),
  '注册成功！'
)

// 合并规则
Object.assign(registerRules, commonRules)
</script>

<style scoped>
.register-container {
  display: flex;
  min-height: 100vh;
}

.decoration-content {
  text-align: center;
}

.decoration-content p {
  font-size: 1.2rem;
  margin-bottom: 2rem;
  opacity: 0.9;
}

/* 右侧注册表单区 */
.register-form-container {
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

.register-form-wrapper {
  width: 100%;
  max-width: 360px;
  padding: 2rem;
}

/* Logo */
.register-logo {
  display: flex;
  flex-direction: column;
  align-items: center;
  margin-bottom: 2rem;
}

/* 验证码容器 */
.captcha-container {
  display: flex;
  gap: 10px;
}

/* 验证码按钮 */
.captcha-button {
  width: 150px;
  background-color: #1660ab;
  border-color: #1660ab;
}

.captcha-button:disabled {
  background-color: rgba(22, 96, 171, 0.6);
  border-color: rgba(22, 96, 171, 0.6);
}

.captcha-button:hover {
  transform: translateY(-1px);
  box-shadow: 0 5px 15px rgba(22, 96, 171, 0.4);
}

.register-button {
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

.register-button:hover {
  transform: translateY(-1px);
  box-shadow: 0 5px 15px rgba(22, 96, 171, 0.4);
}

/* 登录入口 */
.login-entry {
  text-align: center;
  color: #666;
  font-size: 0.9rem;
  margin-top: 2rem;
  padding-top: 1.5rem;
  border-top: 1px solid #f0f0f0;
}

.login-entry .el-link {
  font-weight: 600;
  margin-left: 0.5rem;
  font-size: 1rem;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .register-container {
    flex-direction: column;
  }

  .decoration-content p {
    font-size: 1rem;
  }

  .register-form-container {
    flex: 1;
    width: 100%;
    box-shadow: none;
  }

  .register-form-wrapper {
    max-width: 100%;
    padding: 1.5rem;
  }
}
</style>