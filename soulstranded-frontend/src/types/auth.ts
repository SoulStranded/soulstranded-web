// 修改密码表单类型
export interface ForgetPasswordForm {
  email: string
  password: string
  confirmPassword: string
  captcha: string
}

// 注册表单类型
export interface RegisterForm extends ForgetPasswordForm {
  username: string
}

// 登录表单类型
export interface LoginForm {
  email: string
  password: string
  rememberMe: boolean
}

// 登录响应类型
export interface LoginResponse {
  token: string
  user: {
    id: number
    username: string
    email: string
    avatar?: string
  }
}
