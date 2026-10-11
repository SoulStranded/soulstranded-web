/// 业务状态码（与前端约定）
pub const SUCCESS: i32 = 0;

// 用户 & 认证相关 (1000 ~ 1999)
pub const VALIDATION_FAILED: i32 = 1000; // 参数/表单错误
pub const CAPTCHA_ERROR: i32 = 1001; // 验证码错误
pub const EMAIL_EXISTS: i32 = 1002; // 邮箱已存在
pub const USER_NOT_FOUND: i32 = 1003; // 用户不存在
pub const PASSWORD_ERROR: i32 = 1007; // 密码错误
pub const TOKEN_INVALID: i32 = 1009; // Token 无效

// 权限相关 (4000 ~ 4999)
pub const UNAUTHORIZED: i32 = 4001; // 未登录
pub const FORBIDDEN: i32 = 4002; // 权限不足

// 系统错误 (5000 ~ 5999)
pub const SYSTEM_ERROR: i32 = 5000; // 系统内部错误
