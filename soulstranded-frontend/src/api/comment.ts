import request from '@/utils/request'
import type { Comment, CreateCommentForm, CommentQueryParams, DeleteCommentForm, UpdateCommentForm } from '@/types/comment'
import type { PaginatedResponse } from '@/types/post'

export const commentApi = {
    create(data: CreateCommentForm): Promise<null> {
        return request.post('/post/create-comment', data)
    },
    getComments(params: CommentQueryParams): Promise<PaginatedResponse<Comment>> {
        return request.get('/post/get-comments', { params })
    },
    delete(data: DeleteCommentForm): Promise<null> {
        return request.post('/post/delete-comment', data)
    },
    update(data: UpdateCommentForm): Promise<null> {
        return request.post('/post/update-comment', data)
    }
}
