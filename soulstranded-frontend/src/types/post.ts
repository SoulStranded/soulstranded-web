// 发表帖子表单
export interface PublishPostForm {
    content: string
}

// 分页查询参数
export interface PostQueryParams {
    page: number
    pageSize: number
    userId?: number
}

// 分页响应数据
export interface PaginatedResponse<T> {
    data: T[]
    total: number
    page: number
    pageSize: number
}

// 帖子列表项
export interface Post {
    id: number
    username: string
    content: string
    pubTime: string
    avatar?: string
    likeCount: number
    isLiked: boolean
    commentCount: number
}