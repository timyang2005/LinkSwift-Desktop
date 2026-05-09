import React, { useState } from 'react'

interface LinkInputProps {
  onParse: (url: string) => void
  isLoading?: boolean
}

export function LinkInput({ onParse, isLoading = false }: LinkInputProps) {
  const [url, setUrl] = useState('')
  const [error, setError] = useState<string | null>(null)

  const handleParse = () => {
    if (!url.trim()) {
      setError('请输入分享链接')
      return
    }
    setError(null)
    onParse(url.trim())
  }

  const handlePaste = (e: React.ClipboardEvent) => {
    e.preventDefault()
    const text = e.clipboardData.getData('text')
    setUrl(text)
    if (text.trim()) {
      setError(null)
      onParse(text.trim())
    }
  }

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter') {
      handleParse()
    }
  }

  return (
    <div className="link-input" data-testid="link-input">
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
      <button
        onClick={handleParse}
        disabled={isLoading}
        data-testid="parse-button"
      >
        {isLoading ? '解析中...' : '解析链接'}
      </button>
      {error && <div data-testid="link-error" className="error">{error}</div>}
    </div>
  )
}
