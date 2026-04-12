import { ref } from 'vue'
import { ElMessage } from 'element-plus'
import type { FormInstance, FormRules } from 'element-plus'
import { useRouter } from 'vue-router'
import { authApi } from '@/api'
import { useCountdown } from './useCountdown'

export function useAuthForm<T extends { email: string; captcha: string; password: string; confirmPassword: string }>(
  formData: T,
  formRef: { value: FormInstance | null },
  submitAction: (data: T) => Promise<void>,
  successMessage: string
) {
  const router = useRouter()
  const isSubmitting = ref(false)
  const isCaptchaSending = ref(false)
  const { countdown: captchaCountdown, start: startCaptchaCountdown } = useCountdown(60)

  const commonRules = {
    email: [
      { required: true, message: '请输入邮箱', trigger: 'blur' },
      { type: 'email', message: '请输入正确的邮箱格式', trigger: 'blur' }
    ],
    captcha: [
      { required: true, message: '请输入验证码', trigger: 'blur' },
      { min: 6, max: 6, message: '验证码长度为6位', trigger: 'blur' }
    ],
    password: [
      { required: true, message: '请输入密码', trigger: 'blur' },
      { min: 6, max: 30, message: '密码长度在 6 到 30 个字符', trigger: 'blur' }
    ],
    confirmPassword: [
      { required: true, message: '请确认密码', trigger: 'blur' },
      {
        validator: (rule: any, value: string, callback: any) => {
          if (value !== formData.password) {
            callback(new Error('两次输入密码不一致'))
          } else {
            callback()
          }
        },
        trigger: 'blur'
      }
    ]
  } as FormRules

  const sendCaptcha = async () => {
    if (!formData.email) {
      ElMessage.warning('请先输入邮箱')
      return
    }

    if (isCaptchaSending.value) return

    try {
      isCaptchaSending.value = true
      await authApi.sendCaptcha(formData.email)
      ElMessage.success('验证码发送成功')
      startCaptchaCountdown()
    } finally {
      isCaptchaSending.value = false
    }
  }

  const handleSubmit = async () => {
    if (!formRef.value) return

    try {
      await formRef.value.validate()
      isSubmitting.value = true
      await submitAction(formData)
      ElMessage.success(successMessage)
      router.push('/auth/login')
    } catch (error) {
    } finally {
      isSubmitting.value = false
    }
  }

  const handleLogin = () => {
    router.push('/auth/login')
  }

  return {
    isSubmitting,
    isCaptchaSending,
    captchaCountdown,
    commonRules,
    sendCaptcha,
    handleSubmit,
    handleLogin
  }
}
