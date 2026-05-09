import { describe, test, expect } from 'vitest'
import {
  formatFileSize,
  formatTimestamp,
  parseShareUrl,
  isValidUrl,
  getErrorMessage,
} from '../src/lib/utils'

describe('formatFileSize', () => {
  test('0 字节', () => {
    expect(formatFileSize(0)).toBe('0 B')
  })

  test('字节', () => {
    expect(formatFileSize(512)).toBe('512 B')
  })

  test('KB', () => {
    expect(formatFileSize(1024)).toBe('1.0 KB')
  })

  test('MB', () => {
    expect(formatFileSize(1048576)).toBe('1.0 MB')
  })

  test('GB', () => {
    expect(formatFileSize(1073741824)).toBe('1.0 GB')
  })

  test('TB', () => {
    expect(formatFileSize(1099511627776)).toBe('1.0 TB')
  })

  test('1.5 GB', () => {
    expect(formatFileSize(1610612736)).toBe('1.5 GB')
  })

  test('12.3 GB', () => {
    expect(formatFileSize(13207024435)).toBe('12.3 GB')
  })
})

describe('formatTimestamp', () => {
  test('0 返回占位符', () => {
    expect(formatTimestamp(0)).toBe('-')
  })

  test('有效时间戳', () => {
    const result = formatTimestamp(1700000000)
    expect(result).not.toBe('-')
    expect(typeof result).toBe('string')
  })
})

describe('parseShareUrl', () => {
  test('有效的夸克分享链接', () => {
    expect(parseShareUrl('https://pan.quark.cn/s/abc123')).toBe('abc123')
  })

  test('带尾部斜杠的链接', () => {
    expect(parseShareUrl('https://pan.quark.cn/s/abc123/')).toBe('abc123')
  })

  test('带查询参数的链接', () => {
    expect(parseShareUrl('https://pan.quark.cn/s/abc123?ref=test')).toBe('abc123')
  })

  test('http 协议', () => {
    expect(parseShareUrl('http://pan.quark.cn/s/xyz789')).toBe('xyz789')
  })

  test('无效域名', () => {
    expect(parseShareUrl('https://example.com/s/abc123')).toBeNull()
  })

  test('非 URL 字符串', () => {
    expect(parseShareUrl('not-a-url')).toBeNull()
  })

  test('空字符串', () => {
    expect(parseShareUrl('')).toBeNull()
  })

  test('缺少分享 ID', () => {
    expect(parseShareUrl('https://pan.quark.cn/s/')).toBeNull()
  })
})

describe('isValidUrl', () => {
  test('有效 URL', () => {
    expect(isValidUrl('https://pan.quark.cn/s/abc')).toBe(true)
  })

  test('http URL', () => {
    expect(isValidUrl('http://localhost:6800')).toBe(true)
  })

  test('无效 URL', () => {
    expect(isValidUrl('not-a-url')).toBe(false)
  })

  test('空字符串', () => {
    expect(isValidUrl('')).toBe(false)
  })
})

describe('getErrorMessage', () => {
  test('Error 对象', () => {
    expect(getErrorMessage(new Error('test error'))).toBe('test error')
  })

  test('字符串', () => {
    expect(getErrorMessage('string error')).toBe('string error')
  })

  test('其他类型', () => {
    expect(getErrorMessage(123)).toBe('未知错误')
  })

  test('null', () => {
    expect(getErrorMessage(null)).toBe('未知错误')
  })

  test('undefined', () => {
    expect(getErrorMessage(undefined)).toBe('未知错误')
  })
})
