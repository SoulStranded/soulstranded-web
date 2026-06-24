# SoulStranded

一个现代化的全栈 Web 应用，包含用户认证、帖子管理等功能。

## 项目简介

SoulStranded 是一个基于前后端分离架构的 Web 应用，提供用户注册、登录、发布帖子、查看帖子列表等核心功能。项目采用现代化的技术栈，注重代码质量和开发体验。

## 在线访问

🌐 **线上地址**: https://www.soulstranded.top

可以直接访问上述网址体验完整功能。

## 技术栈

### 前端 (soulstranded-frontend)
- **框架**: Vue 3.5.25
- **语言**: TypeScript 5.9.0
- **构建工具**: Vite 8.0.1
- **UI 组件库**: Element Plus 2.12.0
- **状态管理**: Pinia 3.0.4
- **路由**: Vue Router 4.6.3
- **HTTP 客户端**: Axios 1.13.2

### 后端 (soulstranded-backend)
- **语言**: Rust (Edition 2024)
- **Web 框架**: Axum 0.8.8
- **ORM**: SeaORM 1.1.19
- **数据库**: MySQL 8.0
- **缓存**: Redis
- **异步运行时**: Tokio 1.48.0
- **认证**: JWT (jsonwebtoken 10.2.0)

## 项目结构

```
soulstranded/
├── soulstranded-frontend/    # 前端项目 (Vue 3 + TypeScript)
├── soulstranded-backend/     # 后端项目 (Rust + Axum)
└── README.md                 # 项目说明文档
```

## 核心功能

- 用户认证系统
  - 用户注册（邮箱验证码）
  - 用户登录（JWT 认证）
  - 密码加密存储
  - 重置密码
- 用户中心
  - 更新用户信息
  - 上传头像
  - 修改密码
  - 用户统计
- 帖子管理
  - 发布帖子
  - 查看帖子列表（分页、支持按用户筛选）
  - 按时间倒序排列
  - 点赞帖子
  - 删除帖子
  - 更新帖子
- 评论系统
  - 发表评论
  - 查看评论列表（分页）
  - 删除评论
  - 更新评论
- 响应式设计
- 路由权限控制
- Redis 缓存支持

## 快速开始

### 环境要求

- Node.js ^20.19.0 || >=22.12.0
- Rust 1.70+
- MySQL 8.0+
- Redis 6+

### 安装依赖

#### 前端
```bash
cd soulstranded-frontend
pnpm install
```

#### 后端
```bash
cd soulstranded-backend
cargo build
```

### 配置环境变量

#### 后端配置 (soulstranded-backend/.env)
```env
# 数据库连接配置
# MySQL 连接字符串
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

### 启动项目

#### 前端开发服务器
```bash
cd soulstranded-frontend
pnpm run dev
```
访问 http://localhost:5173

#### 后端服务器
```bash
cd soulstranded-backend
cargo run
```
访问 http://localhost:3000

## 开发指南

### 前端开发

详细的开发文档请查看 [soulstranded-frontend/README.md](./soulstranded-frontend/README.md)

```bash
cd soulstranded-frontend

# 开发模式
pnpm run dev

# 类型检查
pnpm run type-check

# 构建生产版本
pnpm run build

# 预览生产构建
pnpm run preview
```

### 后端开发

详细的开发文档请查看 [soulstranded-backend/README.md](./soulstranded-backend/README.md)

```bash
cd soulstranded-backend

# 检查代码
cargo check

# 运行项目
cargo run

# 运行测试
cargo test

# 代码检查
cargo clippy
```

## 部署

项目提供一键部署脚本 [deploy.sh](./deploy.sh)，用于构建前后端并上传到远程服务器。

### 前置要求

- 本地已安装 `sshpass`
- 本地已安装 Rust 工具链（`cargo`）和 Node.js/pnpm
- 远程服务器已配置好 systemd 服务：`soulstranded-backend` 和 `nginx`
- 远程服务器目标目录已存在：
  - `/opt/soulstranded-backend/`
  - `/var/www/soulstranded-frontend/`

### 使用方法

在项目根目录执行：

```bash
./deploy.sh <username> <password>
```

例如：

```bash
./deploy.sh root your_password
```

### 部署流程

1. 使用 `cargo build --release` 构建后端
2. 使用 `pnpm run build` 构建前端
3. 通过 `sshpass` + `scp` 上传可执行文件和静态资源到远程服务器
4. 远程重启 `soulstranded-backend` 和 `nginx` 服务

### 安全建议

当前脚本通过命令行参数传递服务器密码，仅适合个人或受信任环境使用。生产环境建议改用 SSH Key 认证，并移除 `sshpass` 和 `StrictHostKeyChecking=no` 相关逻辑。

## API 接口

### 认证相关
- `POST /auth/captcha` - 发送验证码
- `POST /auth/register` - 用户注册
- `POST /auth/login` - 用户登录
- `POST /auth/reset-password` - 重置密码

### 用户相关
- `PATCH /user/update-info` - 更新用户信息（需要认证）
- `POST /user/upload-avatar` - 上传头像（需要认证）
- `GET /user/statistics` - 获取用户统计（需要认证）
- `PATCH /user/change-password` - 修改密码（需要认证）

### 帖子相关
- `POST /post/publish` - 发布帖子（需要认证）
- `GET /post/get-posts?page=1&pageSize=10&userId=xxx` - 获取帖子列表（userId 可选）
- `POST /post/like` - 点赞帖子（需要认证）
- `POST /post/delete` - 删除帖子（需要认证）
- `POST /post/update` - 更新帖子（需要认证）

### 评论相关
- `POST /post/create-comment` - 发表评论（需要认证）
- `GET /post/get-comments?postId=1&page=1&pageSize=10` - 获取评论列表
- `POST /post/delete-comment` - 删除评论（需要认证）
- `POST /post/update-comment` - 更新评论（需要认证）

详细的 API 文档请查看 [soulstranded-backend/README.md](./soulstranded-backend/README.md)

## 项目特色

- **前后端分离**: 清晰的架构，独立开发和部署
- **类型安全**: 前端使用 TypeScript，后端使用 Rust
- **现代化技术栈**: 采用最新的框架和工具
- **完善的认证**: JWT 认证 + 密码加密
- **响应式设计**: 支持多种设备

## 开发规范

### 命名约定
- 数据库字段：蛇形命名（snake_case）
- API 响应字段：小驼峰命名（camelCase）
- Rust 变量/函数：蛇形命名（snake_case）
- Rust 结构体：大驼峰命名（PascalCase）
- Vue 组件：大驼峰命名（PascalCase）

### 代码组织
- 前端：组件化开发，使用 Composition API
- 后端：分层架构（handlers → services → database）
- 统一的错误处理和日志记录
