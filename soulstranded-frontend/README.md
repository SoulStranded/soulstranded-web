# Soulstranded Frontend

一个基于 Vue 3 + TypeScript 的现代化前端应用，提供用户认证和帖子管理功能。

## 技术栈

- **框架**: Vue 3.5.25
- **语言**: TypeScript 5.9.0
- **构建工具**: Vite 8.0.1
- **UI 组件库**: Element Plus 2.12.0
- **状态管理**: Pinia 3.0.4
- **路由**: Vue Router 4.6.3
- **HTTP 客户端**: Axios 1.13.2
- **开发工具**: Vue DevTools

## 功能特性

- 用户认证系统
  - 用户注册
  - 用户登录
  - 验证码发送
  - 重置密码
- 用户中心
  - 更新用户信息
  - 上传头像
  - 修改密码
  - 用户统计
- 帖子管理
  - 发布帖子
  - 查看帖子列表（分页）
  - 点赞帖子
  - 删除帖子
  - 更新帖子
- 响应式设计
- 路由权限控制

## 开始使用

### 环境要求

- Node.js ^20.19.0 || >=22.12.0
- npm 或 yarn 或 pnpm

### 安装依赖

```bash
pnpm install
```

### 开发模式

```bash
pnpm run dev
```

开发服务器将在 http://localhost:5173 启动

### 类型检查

```bash
pnpm run type-check
```

### 构建生产版本

```bash
pnpm run build
```

### 预览生产构建

```bash
pnpm run preview
```

## 项目结构

```
src/
├── api/              # API 请求封装
│   ├── auth.ts      # 认证相关 API
│   ├── post.ts      # 帖子相关 API
│   ├── user.ts      # 用户相关 API
│   └── index.ts     # 导出所有 API
├── assets/          # 静态资源
│   └── default-avatar.jpg
├── components/      # 公共组件
│   ├── LayoutWrapper.vue
│   ├── Logo.vue
│   ├── NavigationBar.vue
│   └── Pagination.vue
├── composables/     # 组合式函数（可复用逻辑）
│   ├── useAuthForm.ts   # 认证表单逻辑
│   ├── useCountdown.ts  # 倒计时逻辑
│   └── useNavigation.ts # 导航逻辑
├── constants/       # 常量定义
│   └── errorCode.ts # 错误代码定义
├── layouts/         # 布局组件
│   ├── AuthLayout.vue      # 认证页面布局
│   ├── HomeLayout.vue      # 主页面布局
│   └── UserCenterLayout.vue # 用户中心布局
├── router/          # 路由配置
│   ├── index.ts     # 路由主配置
│   ├── auth.ts      # 认证相关路由
│   ├── post.ts      # 帖子相关路由
│   └── user.ts      # 用户相关路由
├── stores/          # 全局状态管理（Pinia）
│   └── user.ts      # 用户状态管理
├── types/           # TypeScript 类型定义
│   ├── api.ts       # API 相关类型
│   ├── auth.ts      # 认证相关类型
│   ├── post.ts      # 帖子相关类型
│   ├── user.ts      # 用户相关类型
│   └── index.ts     # 导出所有类型
├── utils/           # 工具函数
│   ├── delay.ts     # 延迟工具
│   └── request.ts   # 请求工具
├── views/           # 页面组件
│   ├── Auth/        # 认证相关页面
│   │   ├── Login.vue
│   │   ├── Register.vue
│   │   └── ForgetPassword.vue
│   ├── Post/        # 帖子相关页面
│   │   ├── Home.vue
│   │   └── PublishPost.vue
│   └── User/        # 用户相关页面
│       ├── BaseInfo.vue
│       ├── AccountSecurity.vue
│       └── Statistics.vue
├── App.vue          # 根组件
└── main.ts          # 应用入口
```

## 开发指南

### API 使用

```typescript
import { authApi, postApi } from '@/api'

// 登录
const result = await authApi.login({ username, password })

// 获取帖子列表
const posts = await postApi.getPosts({ page: 1, pageSize: 10 })

// 发布帖子
await postApi.publishPost({ content: 'Hello World' })
```

### Composables 使用

```typescript
import { useNavigation } from '@/composables/useNavigation'

const { activeButton, navigate: handleNavClick } = useNavigation()

// 点击导航按钮
handleNavClick('首页')
```

### 工具函数使用

```typescript
import { formatDate, isValidEmail } from '@/utils'

// 格式化日期
const timeStr = formatDate(new Date())

// 验证邮箱
const valid = isValidEmail(email)
```

### 类型定义

```typescript
import type { User, Post, LoginForm } from '@/types'

const user: User = { ... }
const loginForm: LoginForm = { ... }
```

## 配置说明

### API 代理

开发环境下，`/api` 路径会被代理到 `http://localhost:3000`：

```typescript
// vite.config.ts
proxy: {
  '/api': {
    target: 'http://localhost:3000',
    changeOrigin: true,
    rewrite: (path) => path.replace(/^\/api/, '')
  }
}
```

### 路由配置

- `/` - 重定向到 `/post`
- `/post` - 首页（帖子列表）
- `/auth/login` - 登录页
- `/auth/register` - 注册页
- `/auth/forget-password` - 忘记密码页
- `/post/publish` - 发布帖子（需要登录）
- `/user` - 用户中心（需要登录）

## 构建部署

构建后的文件将输出到 `dist` 目录，可以部署到任何静态文件服务器。

```bash
pnpm run build
```

## 开发规范

- 使用 TypeScript 进行类型检查
- 遵循 Vue 3 Composition API 最佳实践
- 使用 Pinia 进行状态管理
- 组件命名采用 PascalCase
- 文件命名采用 PascalCase（组件）或 camelCase（工具函数）
