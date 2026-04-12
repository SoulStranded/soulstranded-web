// 统一的 API 响应结构
export interface ApiResponse<T = any> {
  code: number
  message: string
  data: T | null
}
