import { describe, test, expect, vi } from 'vitest'
import { render, screen, fireEvent } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { LinkInput } from '../../src/components/LinkInput'

describe('LinkInput 组件', () => {
  test('渲染输入框和解析按钮', () => {
    render(<LinkInput onParse={vi.fn()} />)
    expect(screen.getByTestId('link-url-input')).toBeInTheDocument()
    expect(screen.getByTestId('parse-button')).toBeInTheDocument()
    expect(screen.getByText('解析链接')).toBeInTheDocument()
  })

  test('输入 URL 后点击解析按钮调用 onParse', async () => {
    const onParse = vi.fn()
    render(<LinkInput onParse={onParse} />)
    const input = screen.getByTestId('link-url-input')
    await userEvent.type(input, 'https://pan.quark.cn/s/abc123')
    fireEvent.click(screen.getByTestId('parse-button'))
    expect(onParse).toHaveBeenCalledWith('https://pan.quark.cn/s/abc123')
  })

  test('空输入点击解析显示错误', () => {
    render(<LinkInput onParse={vi.fn()} />)
    fireEvent.click(screen.getByTestId('parse-button'))
    expect(screen.getByTestId('link-error')).toBeInTheDocument()
    expect(screen.getByText('请输入分享链接')).toBeInTheDocument()
  })

  test('加载中状态禁用按钮', () => {
    render(<LinkInput onParse={vi.fn()} isLoading={true} />)
    expect(screen.getByTestId('parse-button')).toBeDisabled()
    expect(screen.getByText('解析中...')).toBeInTheDocument()
  })

  test('Enter 键触发解析', async () => {
    const onParse = vi.fn()
    render(<LinkInput onParse={onParse} />)
    const input = screen.getByTestId('link-url-input')
    await userEvent.type(input, 'https://pan.quark.cn/s/abc{enter}')
    expect(onParse).toHaveBeenCalled()
  })

  test('输入框有正确的 placeholder', () => {
    render(<LinkInput onParse={vi.fn()} />)
    expect(screen.getByPlaceholderText('粘贴夸克网盘分享链接')).toBeInTheDocument()
  })

  test('输入内容后清除错误', async () => {
    render(<LinkInput onParse={vi.fn()} />)
    fireEvent.click(screen.getByTestId('parse-button'))
    expect(screen.getByTestId('link-error')).toBeInTheDocument()
    const input = screen.getByTestId('link-url-input')
    await userEvent.type(input, 'https://pan.quark.cn/s/abc')
    expect(screen.queryByTestId('link-error')).not.toBeInTheDocument()
  })
})
