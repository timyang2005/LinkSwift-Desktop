/**
 * LinkSwift Desktop - 工具函数库
 * 
 * 提供常用的工具函数，包括文件大小格式化、时间戳转换、URL 解析等
 */

/**
 * 格式化文件大小为可读字符串
 * @param bytes - 文件大小（字节）
 * @returns 格式化后的大小字符串，如 "1.5 MB"
 */
export function formatFileSize(bytes: number): string {
  if (bytes === 0) return '0 B'
  const units = ['B', 'KB', 'MB', 'GB', 'TB']
  const k = 1024
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  const value = bytes / Math.pow(k, i)
  return `${value.toFixed(i > 0 ? 1 : 0)} ${units[i]}`
}

/**
 * 格式化时间戳为本地化日期时间字符串
 * @param ts - Unix 时间戳（秒）
 * @returns 格式化的日期时间字符串，使用中文本地化格式
 */
export function formatTimestamp(ts: number): string {
  if (ts === 0) return '-'
  return new Date(ts * 1000).toLocaleString('zh-CN')
}

/**
 * 从分享链接中解析提取分享 ID
 * @param url - 夸克网盘分享链接
 * @returns 分享 ID（s/ 后面的部分），如果解析失败返回 null
 */
export function parseShareUrl(url: string): string | null {
  const pattern = /https?:\/\/pan\.quark\.cn\/s\/([a-zA-Z0-9]+)/
  const match = url.match(pattern)
  return match ? match[1] : null
}

/**
 * 验证字符串是否为有效的 URL
 * @param url - 待验证的字符串
 * @returns 是否为有效 URL
 */
export function isValidUrl(url: string): boolean {
  try {
    new URL(url)
    return true
  } catch {
    return false
  }
}

/**
 * 生成唯一的 UUID 字符串
 * @returns 随机生成的 UUID
 */
export function generateId(): string {
  return crypto.randomUUID()
}

/**
 * 从任意错误对象中提取错误消息字符串
 * @param error - 错误对象（可能是 Error 实例、字符串或其他类型）
 * @returns 错误消息字符串
 */
export function getErrorMessage(error: unknown): string {
  if (error instanceof Error) return error.message
  if (typeof error === 'string') return error
  return '未知错误'
}
