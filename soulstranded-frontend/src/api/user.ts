import request from '@/utils/request'
import type { UpdateUserInfoRequest, UploadAvatarResponse, ChangePasswordRequest } from '@/types/user'

export interface PostStatisticsResponse {
  postCount: number
  likesReceived: number
}

export const userApi = {
  // 更新用户信息
  updateUserInfo(data: UpdateUserInfoRequest): Promise<null> {
    return request.patch('/user/update-info', data)
  },

  // 上传头像
  uploadAvatar(file: File): Promise<UploadAvatarResponse> {
    const formData = new FormData()
    formData.append('avatar', file)
    return request.post('/user/upload-avatar', formData)
  },

  // 获取帖子统计
  getPostStatistics(): Promise<PostStatisticsResponse> {
    return request.get('/user/statistics')
  },

  // 修改密码
  changePassword(data: ChangePasswordRequest): Promise<null> {
    return request.patch('/user/change-password', data)
  },
}
