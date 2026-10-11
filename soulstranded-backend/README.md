# SoulStranded Backend

SoulStranded 后端服务，基于 Rust + Axum + SeaORM 构建的现代化 Web 应用后端。

## 技术栈

- **语言**: Rust (Edition 2024)
- **Web 框架**: Axum 0.8.8
- **ORM**: SeaORM 1.1.19
- **数据库**: MySQL 8.0
- **缓存**: Redis
- **异步运行时**: Tokio 1.48.0
- **认证**: JWT (jsonwebtoken 10.2.0)
- **密码加密**: bcrypt 0.17.1
- **邮件服务**: lettre 0.11.19
- **日志**: tracing 0.1.44

## 主要功能

- 用户注册与登录（邮箱验证码）
- JWT 身份认证
- 重置密码
- 用户信息管理
- 头像上传
- 用户统计
- 帖子发布
- 帖子列表查询（支持分页）
- 帖子点赞
- 帖子删除与更新
- 数据库关系查询
- Redis 缓存支持
- 请求日志记录

## 项目结构

```
src/
├── config/          # 配置管理
├── constants.rs     # 常量定义（错误码等）
├── database/        # 数据库连接与操作
├── entity/          # SeaORM 实体模型
├── handlers/        # API 处理器
├── lib.rs           # 库入口文件
├── main.rs          # 应用入口文件
├── middleware/      # 中间件（日志等）
├── redis/           # Redis 连接与操作
├── routes/          # 路由配置
├── services/        # 业务逻辑层
└── utils/           # 工具函数（验证、序列化等）
```

## 环境配置

在项目根目录创建 `.env` 文件：

```env
# 数据库连接配置
DATABASE_URL=mysql://your_username_or_root:your_password@127.0.0.1:3306/soulstranded

# redis 连接配置
REDIS_URL=redis://127.0.0.1:6379

# 服务器配置
PORT=3000
HOST=0.0.0.0

# 邮箱配置
SMTP_URL=smtp.example.com
SMTP_USERNAME=your_email@example.com
SMTP_PASSWORD=your_smtp_password

# JWT 配置
JWT_SECRET=your_jwt_secret_key
JWT_EXPIRATION_HOURS=24
JWT_REMEMBER_EXPIRATION_HOURS=720

# 上传文件路径
AVATAR_UPLOAD_DIR=./uploads/avatars
```

## 快速开始

### 前置要求

- Rust 1.98+（Edition 2024，建议用 mise 管理：`mise install`）
- MySQL 8.0+
- Redis 6+

### 安装依赖

```bash
cargo build
```

### 运行项目

```bash
cargo run
```

服务器将在 `http://0.0.0.0:3000` 启动。

### 开发模式

```bash
cargo check          # 检查代码
cargo clippy         # 代码检查
cargo test           # 运行测试
```

## API 接口

### 认证相关

#### 发送验证码
```
POST /auth/captcha
Content-Type: application/json

{
  "email": "user@example.com"
}
```

#### 用户注册
```
POST /auth/register
Content-Type: application/json

{
  "username": "SoulStranded",
  "email": "user@example.com",
  "password": "password123",
  "confirmPassword": "password123",
  "captcha": "123456"
}
```

#### 用户登录
```
POST /auth/login
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "password123",
  "rememberMe": false
}
```

#### 重置密码
```
POST /auth/reset-password
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "newPassword123",
  "confirmPassword": "newPassword123",
  "captcha": "123456"
}
```

### 用户相关

#### 更新用户信息
```
PATCH /user/update-info
Content-Type: application/json
Authorization: Bearer <token>

{
  "username": "NewUsername"
}
```

#### 上传头像
```
POST /user/upload-avatar
Content-Type: multipart/form-data
Authorization: Bearer <token>

avatar: <file>
```

#### 获取用户统计
```
GET /user/statistics
Authorization: Bearer <token>
```

#### 修改密码
```
PATCH /user/change-password
Content-Type: application/json
Authorization: Bearer <token>

{
  "oldPassword": "oldPassword123",
  "newPassword": "newPassword123",
  "confirmPassword": "newPassword123"
}
```

### 帖子相关

#### 发布帖子
```
POST /post/publish
Content-Type: application/json
Authorization: Bearer <token>

{
  "content": "这是一篇测试用的帖子"
}
```

#### 获取帖子列表
```
GET /post/get-posts?page=1&pageSize=10
Authorization: Bearer <token>
```

响应示例：
```json
{
  "code": 0,
  "message": "success",
  "data": {
    "page": 1,
    "pageSize": 10,
    "total": 2,
    "data": [
      {
        "id": 2,
        "content": "这是第二遍测试用的帖子",
        "pubTime": "2026-01-03 22:42:12",
        "username": "SoulStranded"
      },
      {
        "id": 1,
        "content": "这是一篇测试用的帖子",
        "pubTime": "2026-01-03 22:36:10",
        "username": "SoulStranded"
      }
    ]
  }
}
```

#### 点赞帖子
```
POST /post/like
Content-Type: application/json
Authorization: Bearer <token>

{
  "postId": 1
}
```

#### 删除帖子
```
POST /post/delete
Content-Type: application/json
Authorization: Bearer <token>

{
  "postId": 1
}
```

#### 更新帖子
```
POST /post/update
Content-Type: application/json
Authorization: Bearer <token>

{
  "postId": 1,
  "content": "更新后的帖子内容"
}
```

## 数据库模型

### Users 表
- `id`: 主键
- `username`: 用户名
- `email`: 邮箱（唯一）
- `password`: 密码（bcrypt 加密）
- `avatar`: 头像路径

### Contents 表
- `id`: 主键
- `content`: 帖子内容
- `pub_time`: 发布时间
- `user_id`: 用户 ID（外键）

### Likes 表
- `id`: 主键
- `user_id`: 用户 ID（外键）
- `content_id`: 帖子 ID（外键）

## 错误码说明

| 错误码 | 说明 |
|--------|------|
| 0 | 成功 |
| 1000 | 参数/表单错误 |
| 1001 | 验证码错误 |
| 1002 | 邮箱已存在 |
| 1003 | 用户不存在 |
| 1007 | 密码错误 |
| 1009 | Token 无效 |
| 4001 | 未登录 |
| 4002 | 权限不足 |
| 5000 | 系统内部错误 |

## 开发规范

### 命名约定
- 数据库字段：蛇形命名（snake_case）
- API 响应字段：小驼峰命名（camelCase）
- Rust 变量/函数：蛇形命名（snake_case）
- Rust 结构体：大驼峰命名（PascalCase）

### 代码组织
- **handlers**: 处理 HTTP 请求，调用 services
- **services**: 实现业务逻辑，调用 database/redis
- **database**: 数据库操作，使用 SeaORM
- **utils**: 可复用的工具函数

### 错误处理
- 使用统一的 `ApiResponse` 结构返回响应
- 错误码定义在 `constants.rs` 中
- 日志记录使用 `tracing` crate
