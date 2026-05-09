export function formatFileSize(bytes: number): string {
  if (bytes === 0) return '0 B'
  const units = ['B', 'KB', 'MB', 'GB', 'TB']
  const k = 1024
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  const value = bytes / Math.pow(k, i)
  return `${value.toFixed(i > 0 ? 1 : 0)} ${units[i]}`
}

export function formatTimestamp(ts: number): string {
  if (ts === 0) return '-'
  return new Date(ts * 1000).toLocaleString('zh-CN')
}

export function parseShareUrl(url: string): string | null {
  const pattern = /https?:\/\/pan\.quark\.cn\/s\/([a-zA-Z0-9]+)/
  const match = url.match(pattern)
  return match ? match[1] : null
}

export function isValidUrl(url: string): boolean {
  try {
    new URL(url)
    return true
  } catch {
    return false
  }
}

export function generateId(): string {
  return crypto.randomUUID()
}

export function getErrorMessage(error: unknown): string {
  if (error instanceof Error) return error.message
  if (typeof error === 'string') return error
  return '未知错误'
}
