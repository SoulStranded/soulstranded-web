<template>
  <div class="forget-password-container">
    <!-- 右侧忘记密码表单区 -->
    <div class="forget-password-form-container">
      <div class="forget-password-form-wrapper">
        <!-- Logo -->
        <div class="forget-password-logo">
          <Logo />
        </div>

        <div class="decoration-content">
          <p>找回你的密码</p>
        </div>

        <!-- 忘记密码表单 -->
        <el-form ref="forgetPasswordFormRef" :model="forgetPasswordForm" :rules="forgetPasswordRules"
          class="forget-password-form">

          <el-form-item prop="email">
            <el-input v-model="forgetPasswordForm.email" type="email" placeholder="请输入邮箱" clearable size="large">
              <template #prefix>
                <el-icon>
                  <Message />
                </el-icon>
              </template>
            </el-input>
          </el-form-item>

          <el-form-item prop="captcha">
            <div class="captcha-container">
              <el-input v-model="forgetPasswordForm.captcha" placeholder="请输入验证码" clearable size="large">
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
            <el-input v-model="forgetPasswordForm.password" type="password" placeholder="请输入新密码" show-password clearable
              size="large">
              <template #prefix>
                <el-icon>
                  <Lock />
                </el-icon>
              </template>
            </el-input>
          </el-form-item>

          <el-form-item prop="confirmPassword">
            <el-input v-model="forgetPasswordForm.confirmPassword" type="password" placeholder="请确认新密码" show-password
              clearable size="large" @keyup.enter="handleResetPassword">
              <template #prefix>
                <el-icon>
                  <Lock />
                </el-icon>
              </template>
            </el-input>
          </el-form-item>

          <el-form-item>
            <el-button type="primary" :loading="isResetting" @click="handleResetPassword" size="large"
              class="reset-password-button" block>
              重置密码
            </el-button>
          </el-form-item>
        </el-form>

        <!-- 登录入口 -->
        <div class="login-entry">
          <span>想起密码了？</span>
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
import type { FormInstance } from 'element-plus'
import Logo from '@/components/Logo.vue'
import { Message, Lock } from '@element-plus/icons-vue'
import { authApi } from '@/api'
import { useAuthForm } from '@/composables/useAuthForm'
import type { ForgetPasswordForm } from '@/types/auth'

// 表单引用
const forgetPasswordFormRef = ref<FormInstance | null>(null)

// 忘记密码表单数据
const forgetPasswordForm = reactive<ForgetPasswordForm>({
  email: '',
  password: '',
  confirmPassword: '',
  captcha: ''
})

// 使用 useAuthForm composable
const {
  isSubmitting: isResetting,
  isCaptchaSending,
  captchaCountdown,
  commonRules: forgetPasswordRules,
  sendCaptcha,
  handleSubmit: handleResetPassword,
  handleLogin
} = useAuthForm(
  forgetPasswordForm,
  forgetPasswordFormRef,
  async (data) => {
    await authApi.resetPassword(data)
  },
  '密码重置成功！'
)
</script>

<style scoped>
.forget-password-container {
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

/* 右侧忘记密码表单区 */
.forget-password-form-container {
  flex: 0 0 420px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: white;
  box-shadow: -5px 0 25px rgba(0, 0, 0, 0.1);
  animation: fadeInUp 0.5s ease-out;
  margin: auto 0;
  height: auto;
  padding: 1rem;
  border-radius: 20px;
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

.forget-password-form-wrapper {
  width: 100%;
  max-width: 360px;
  padding: 2rem;
}

/* Logo */
.forget-password-logo {
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

.reset-password-button {
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

.reset-password-button:hover {
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
  .forget-password-container {
    flex-direction: column;
  }

  .decoration-content p {
    font-size: 1rem;
  }

  .forget-password-form-container {
    flex: 1;
    width: 100%;
    box-shadow: none;
  }

  .forget-password-form-wrapper {
    max-width: 100%;
    padding: 1.5rem;
  }
}
</style>