// 用户更新信息请求体
export interface UpdateUserInfoRequest {
    username?: string
}

export interface UploadAvatarResponse {
    id: number
    username: string
    email: string
    avatar: string
}

export interface ChangePasswordRequest {
    oldPassword: string
    newPassword: string
}