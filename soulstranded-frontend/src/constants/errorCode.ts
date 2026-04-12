export const ERROR_CODE = {
    // 用户 & 认证相关 (1000 ~ 1999)
    SUCCESS: 0,    // 成功
    VALIDATION_FAILED: 1000,    // 参数/表单错误
    CAPTCHA_ERROR: 1001,     // 验证码错误
    EMAIL_EXISTS: 1002,     // 邮箱已存在
    USER_NOT_FOUND: 1003,    // 用户不存在
    PASSWORD_ERROR: 1007,    // 密码错误
    TOKEN_INVALID: 1009,    // token无效

    // 权限相关 (4000 ~ 4999)
    UNAUTHORIZED: 4001,    // 未登录
    FORBIDDEN: 4002,    // 权限不足

    // 系统错误 (5000 ~ 5999)
    SYSTEM_ERROR: 5000,    // 系统内部错误
} as const