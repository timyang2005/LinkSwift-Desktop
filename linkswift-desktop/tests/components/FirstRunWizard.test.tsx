import { describe, test, expect, vi } from 'vitest'
import { render, screen, fireEvent, waitFor, act } from '@testing-library/react'
import { FirstRunWizard } from '../../src/components/FirstRunWizard'

describe('FirstRunWizard 组件', () => {
  test('渲染向导第一步', () => {
    render(
      <FirstRunWizard
        onComplete={vi.fn()}
        onLogin={vi.fn().mockResolvedValue(true)}
        onTestConnection={vi.fn().mockResolvedValue(true)}
      />
    )
    expect(screen.getByTestId('first-run-wizard')).toBeInTheDocument()
    expect(screen.getByTestId('wizard-step-login')).toBeInTheDocument()
  })

  test('第一步显示登录按钮', () => {
    render(
      <FirstRunWizard
        onComplete={vi.fn()}
        onLogin={vi.fn().mockResolvedValue(true)}
        onTestConnection={vi.fn().mockResolvedValue(true)}
      />
    )
    expect(screen.getByTestId('wizard-login-button')).toBeInTheDocument()
  })

  test('未登录时下一步按钮禁用', () => {
    render(
      <FirstRunWizard
        onComplete={vi.fn()}
        onLogin={vi.fn().mockResolvedValue(false)}
        onTestConnection={vi.fn().mockResolvedValue(true)}
      />
    )
    expect(screen.getByTestId('wizard-next')).toBeDisabled()
  })

  test('登录成功后下一步按钮启用', async () => {
    render(
      <FirstRunWizard
        onComplete={vi.fn()}
        onLogin={vi.fn().mockResolvedValue(true)}
        onTestConnection={vi.fn().mockResolvedValue(true)}
      />
    )
    await act(async () => {
      fireEvent.click(screen.getByTestId('wizard-login-button'))
    })
    await waitFor(() => {
      expect(screen.getByTestId('wizard-next')).not.toBeDisabled()
    })
  })

  test('点击下一步进入 RPC 配置步骤', async () => {
    render(
      <FirstRunWizard
        onComplete={vi.fn()}
        onLogin={vi.fn().mockResolvedValue(true)}
        onTestConnection={vi.fn().mockResolvedValue(true)}
      />
    )
    await act(async () => {
      fireEvent.click(screen.getByTestId('wizard-login-button'))
    })
    await waitFor(() => {
      expect(screen.getByTestId('wizard-next')).not.toBeDisabled()
    })
    await act(async () => {
      fireEvent.click(screen.getByTestId('wizard-next'))
    })
    await waitFor(() => {
      expect(screen.getByTestId('wizard-step-rpc')).toBeInTheDocument()
    })
  })

  test('RPC 配置步骤显示输入框', async () => {
    render(
      <FirstRunWizard
        onComplete={vi.fn()}
        onLogin={vi.fn().mockResolvedValue(true)}
        onTestConnection={vi.fn().mockResolvedValue(true)}
      />
    )
    await act(async () => {
      fireEvent.click(screen.getByTestId('wizard-login-button'))
    })
    await waitFor(() => {
      expect(screen.getByTestId('wizard-next')).not.toBeDisabled()
    })
    await act(async () => {
      fireEvent.click(screen.getByTestId('wizard-next'))
    })
    await waitFor(() => {
      expect(screen.getByTestId('wizard-rpc-url')).toBeInTheDocument()
      expect(screen.getByTestId('wizard-rpc-token')).toBeInTheDocument()
    })
  })

  test('上一步按钮返回登录步骤', async () => {
    render(
      <FirstRunWizard
        onComplete={vi.fn()}
        onLogin={vi.fn().mockResolvedValue(true)}
        onTestConnection={vi.fn().mockResolvedValue(true)}
      />
    )
    await act(async () => {
      fireEvent.click(screen.getByTestId('wizard-login-button'))
    })
    await waitFor(() => {
      expect(screen.getByTestId('wizard-next')).not.toBeDisabled()
    })
    await act(async () => {
      fireEvent.click(screen.getByTestId('wizard-next'))
    })
    await waitFor(() => {
      expect(screen.getByTestId('wizard-step-rpc')).toBeInTheDocument()
    })
    await act(async () => {
      fireEvent.click(screen.getByTestId('wizard-prev'))
    })
    await waitFor(() => {
      expect(screen.getByTestId('wizard-step-login')).toBeInTheDocument()
    })
  })

  test('完成步骤调用 onComplete', async () => {
    const onComplete = vi.fn()
    render(
      <FirstRunWizard
        onComplete={onComplete}
        onLogin={vi.fn().mockResolvedValue(true)}
        onTestConnection={vi.fn().mockResolvedValue(true)}
      />
    )
    await act(async () => {
      fireEvent.click(screen.getByTestId('wizard-login-button'))
    })
    await waitFor(() => {
      expect(screen.getByTestId('wizard-next')).not.toBeDisabled()
    })
    await act(async () => {
      fireEvent.click(screen.getByTestId('wizard-next'))
    })
    await waitFor(() => {
      expect(screen.getByTestId('wizard-test-connection')).toBeInTheDocument()
    })
    await act(async () => {
      fireEvent.click(screen.getByTestId('wizard-test-connection'))
    })
    await waitFor(() => {
      expect(screen.getByTestId('wizard-next')).not.toBeDisabled()
    })
    await act(async () => {
      fireEvent.click(screen.getByTestId('wizard-next'))
    })
    await waitFor(() => {
      expect(screen.getByTestId('wizard-step-done')).toBeInTheDocument()
    })
    await act(async () => {
      fireEvent.click(screen.getByTestId('wizard-complete'))
    })
    expect(onComplete).toHaveBeenCalled()
  })

  test('显示步骤进度', () => {
    render(
      <FirstRunWizard
        onComplete={vi.fn()}
        onLogin={vi.fn().mockResolvedValue(true)}
        onTestConnection={vi.fn().mockResolvedValue(true)}
      />
    )
    expect(screen.getByText(/1\/3/)).toBeInTheDocument()
  })
})
