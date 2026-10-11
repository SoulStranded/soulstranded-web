// 发表评论表单
export interface CreateCommentForm {
    content: string
    postId: number
}

export interface DeleteCommentForm {
    commentId: number
}

export interface UpdateCommentForm {
    commentId: number
    content: string
}

export interface CommentQueryParams {
    postId: number
    page: number
    pageSize: number
}

// 评论列表项
export interface Comment {
    id: number
    content: string
    postId: number
    userId: number
    createdAt: string
    updatedAt: string
    username: string
    avatar?: string
}
