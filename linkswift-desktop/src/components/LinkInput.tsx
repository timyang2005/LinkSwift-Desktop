/**
 * LinkSwift Desktop - 链接输入组件
 * 
 * 提供夸克网盘分享链接的输入和解析功能
 */

import { useState } from 'react'

// 链接输入组件的属性接口
interface LinkInputProps {
  onParse: (url: string) => void      // 解析链接的回调
  isLoading?: boolean                 // 是否正在加载（解析中）
}

/**
 * 链接输入组件
 * 提供输入框和按钮用于输入和解析分享链接
 * 支持粘贴自动解析和回车键快捷解析
 * @param onParse - 解析链接的回调函数
 * @param isLoading - 是否正在加载状态
 */
export function LinkInput({ onParse, isLoading = false }: LinkInputProps) {
  // 输入的链接文本
  const [url, setUrl] = useState('')
  // 错误信息
  const [error, setError] = useState<string | null>(null)

  // 处理解析按钮点击
  const handleParse = () => {
    if (!url.trim()) {
      setError('请输入分享链接')
      return
    }
    setError(null)
    onParse(url.trim())
  }

  // 处理粘贴事件，自动解析链接
  const handlePaste = (e: React.ClipboardEvent) => {
    e.preventDefault()
    const text = e.clipboardData.getData('text')
    setUrl(text)
    if (text.trim()) {
      setError(null)
      onParse(text.trim())
    }
  }

  // 处理键盘事件，回车键触发解析
  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter') {
      handleParse()
    }
  }

  return (
    <div className="link-input" data-testid="link-input">
      {/* 链接输入框 */}
      <input
        type="text"
        value={url}
        onChange={(e) => { setUrl(e.target.value); if (e.target.value.trim()) setError(null) }}
        onPaste={handlePaste}
        onKeyDown={handleKeyDown}
        placeholder="粘贴夸克网盘分享链接"
        data-testid="link-url-input"
        aria-label="分享链接输入框"
      />
      {/* 解析按钮 */}
      <button
        onClick={handleParse}
        disabled={isLoading}
        data-testid="parse-button"
      >
        {isLoading ? '解析中...' : '解析链接'}
      </button>
      {/* 错误信息显示 */}
      {error && <div data-testid="link-error" className="error">{error}</div>}
    </div>
  )
}
