import type { RegisterForm, LoginForm, LoginResponse, ForgetPasswordForm } from '@/types/auth'
import request from '@/utils/request'

export const authApi = {
  // 发送验证码（响应拦截器已提取 data，返回 null）
  sendCaptcha(email: string): Promise<null> {
    return request.post('/auth/captcha', { email })
  },

  // 注册（响应拦截器已提取 data，返回 null）
  register(data: RegisterForm): Promise<null> {
    return request.post('/auth/register', data)
  },

  // 登录（响应拦截器已提取 data，返回 LoginResponse）
  login(data: LoginForm): Promise<LoginResponse> {
    return request.post('/auth/login', data)
  },

  // 忘记密码 - 重置密码（响应拦截器已提取 data，返回 null）
  resetPassword(data: ForgetPasswordForm): Promise<null> {
    return request.post('/auth/reset-password', data)
  },
}
