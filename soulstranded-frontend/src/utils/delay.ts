/**
 * 延迟执行函数
 * @param ms 延迟的毫秒数
 * @returns Promise
 */
export const delay = (ms: number): Promise<void> => {
  return new Promise(resolve => setTimeout(resolve, ms))
}

/**
 * 延迟执行回调函数
 * @param callback 要执行的回调函数
 * @param ms 延迟的毫秒数
 * @returns 定时器ID，可用于 clearTimeout 取消
 */
export const setTimeoutCallback = <T = any>(
  callback: () => T,
  ms: number
): number => {
  return setTimeout(callback, ms) as unknown as number
}
