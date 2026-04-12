# SoulStranded

一个现代化的全栈 Web 应用，包含用户认证、帖子管理等功能。

## 项目简介

SoulStranded 是一个基于前后端分离架构的 Web 应用，提供用户注册、登录、发布帖子、查看帖子列表等核心功能。项目采用现代化的技术栈，注重代码质量和开发体验。

## 在线访问

🌐 **线上地址**: https://www.soulstranded.top/

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
- **数据库**: SQLite
- **缓存**: Redis
- **异步运行时**: Tokio 1.48.0
- **认证**: JWT (jsonwebtoken 10.2.0)

## 项目结构

```
soulstranded/
├── soulstranded-frontend/    # 前端项目 (Vue 3 + TypeScript)
├── soulstranded-backend/     # 后端项目 (Rust + Axum)
├── README.md                 # 项目说明文档
└── deploy.sh                 # 部署脚本
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
- SQLite 3+
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
DATABASE_URL=sqlite://data/soulstranded.db

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

## 部署

### 手动部署

进入项目根目录，执行部署脚本：

```bash
./deploy.sh
```

部署脚本会自动执行以下步骤：
1. 拉取最新代码 (`git pull`)
2. 构建后端 (`cargo build --release`)
3. 构建前端 (`pnpm install --frozen-lockfile && pnpm run build`)
4. 部署后端到 `/opt/soulstranded-backend/`
5. 部署前端到 `/var/www/soulstranded-frontend`
6. 重启后端服务 (`systemctl restart soulstranded-backend`)
7. 重启 Nginx (`systemctl restart nginx`)

### 前置要求

部署前请确保：
- 已安装 Rust 工具链
- 已安装 Node.js 和 pnpm
- 已配置 systemd 服务 (`soulstranded-backend`)
- 已配置 Nginx 服务
- 后端环境变量已正确配置

## 项目特色

- **前后端分离**: 清晰的架构，独立开发和部署
- **类型安全**: 前端使用 TypeScript，后端使用 Rust
- **现代化技术栈**: 采用最新的框架和工具
- **完善的认证**: JWT 认证 + 密码加密
- **响应式设计**: 支持多种设备
- **一键部署**: 提供自动化部署脚本

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
