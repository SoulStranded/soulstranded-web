import request from '@/utils/request'
import type { Post, PublishPostForm, PostQueryParams, PaginatedResponse } from '@/types/post'


export const postApi = {
    // 发表帖子
    publish(data: PublishPostForm): Promise<null> {
        return request.post('/post/publish', data)
    },
    // 获取帖子列表（带分页）
    getPosts(params: PostQueryParams): Promise<PaginatedResponse<Post>> {
        return request.get('/post/get-posts', { params })
    },
    // 点赞/取消点赞
    likePost(postId: number): Promise<null> {
        return request.post('/post/like', { postId })
    },
    // 删除帖子
    deletePost(postId: number): Promise<null> {
        return request.post('/post/delete', { postId })
    },
    // 更新帖子
    updatePost(postId: number, content: string): Promise<null> {
        return request.post('/post/update', { postId, content })
    }
}