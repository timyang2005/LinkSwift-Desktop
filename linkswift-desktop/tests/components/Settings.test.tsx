import { describe, test, expect, vi } from 'vitest'
import { render, screen, fireEvent, waitFor } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { Settings } from '../../src/components/Settings'
import type { RpcServer } from '../../src/types'

const mockConfig = {
  credential: { is_valid: true, last_verified: 1700000000 },
  rpc_servers: [
    {
      id: 'srv1',
      name: 'Aria2',
      url: 'http://localhost:6800',
      downloader_type: 'Aria2' as const,
      is_default: true,
    },
  ],
}

describe('Settings 组件', () => {
  test('渲染设置面板', () => {
    render(
      <Settings
        config={mockConfig}
        onAddServer={vi.fn()}
        onDeleteServer={vi.fn()}
        onTestConnection={vi.fn().mockResolvedValue(true)}
        onReLogin={vi.fn()}
      />
    )
    expect(screen.getByTestId('settings-panel')).toBeInTheDocument()
  })

  test('显示登录状态', () => {
    render(
      <Settings
        config={mockConfig}
        onAddServer={vi.fn()}
        onDeleteServer={vi.fn()}
        onTestConnection={vi.fn().mockResolvedValue(true)}
        onReLogin={vi.fn()}
      />
    )
    expect(screen.getByTestId('cookie-status')).toHaveTextContent('已登录')
  })

  test('未登录状态', () => {
    const unauthConfig = {
      credential: { is_valid: false, last_verified: 0 },
      rpc_servers: [],
    }
    render(
      <Settings
        config={unauthConfig}
        onAddServer={vi.fn()}
        onDeleteServer={vi.fn()}
        onTestConnection={vi.fn().mockResolvedValue(false)}
        onReLogin={vi.fn()}
      />
    )
    expect(screen.getByTestId('cookie-status')).toHaveTextContent('未登录')
  })

  test('显示已配置的 RPC 服务器', () => {
    render(
      <Settings
        config={mockConfig}
        onAddServer={vi.fn()}
        onDeleteServer={vi.fn()}
        onTestConnection={vi.fn().mockResolvedValue(true)}
        onReLogin={vi.fn()}
      />
    )
    expect(screen.getByText('Aria2 - http://localhost:6800')).toBeInTheDocument()
  })

  test('删除 RPC 服务器', () => {
    const onDeleteServer = vi.fn()
    render(
      <Settings
        config={mockConfig}
        onAddServer={vi.fn()}
        onDeleteServer={onDeleteServer}
        onTestConnection={vi.fn().mockResolvedValue(true)}
        onReLogin={vi.fn()}
      />
    )
    fireEvent.click(screen.getByTestId('delete-server-srv1'))
    expect(onDeleteServer).toHaveBeenCalledWith('srv1')
  })

  test('重新登录按钮', () => {
    const onReLogin = vi.fn()
    render(
      <Settings
        config={mockConfig}
        onAddServer={vi.fn()}
        onDeleteServer={vi.fn()}
        onTestConnection={vi.fn().mockResolvedValue(true)}
        onReLogin={onReLogin}
      />
    )
    fireEvent.click(screen.getByTestId('re-login-button'))
    expect(onReLogin).toHaveBeenCalled()
  })

  test('添加 RPC 服务器', async () => {
    const onAddServer = vi.fn()
    render(
      <Settings
        config={{ ...mockConfig, rpc_servers: [] }}
        onAddServer={onAddServer}
        onDeleteServer={vi.fn()}
        onTestConnection={vi.fn().mockResolvedValue(true)}
        onReLogin={vi.fn()}
      />
    )
    await userEvent.type(screen.getByTestId('new-server-name'), 'BitComet')
    await userEvent.type(screen.getByTestId('new-server-url'), 'http://localhost:8888')
    fireEvent.click(screen.getByTestId('add-server-button'))
    expect(onAddServer).toHaveBeenCalled()
    const addedServer = onAddServer.mock.calls[0][0] as RpcServer
    expect(addedServer.name).toBe('BitComet')
    expect(addedServer.url).toBe('http://localhost:8888')
  })

  test('测试连接成功', async () => {
    const onTestConnection = vi.fn().mockResolvedValue(true)
    render(
      <Settings
        config={{ ...mockConfig, rpc_servers: [] }}
        onAddServer={vi.fn()}
        onDeleteServer={vi.fn()}
        onTestConnection={onTestConnection}
        onReLogin={vi.fn()}
      />
    )
    await userEvent.type(screen.getByTestId('new-server-url'), 'http://localhost:6800')
    fireEvent.click(screen.getByTestId('test-connection-button'))
    await waitFor(() => {
      expect(screen.getByTestId('test-connection-result')).toHaveTextContent('连接成功')
    })
  })

  test('测试连接失败', async () => {
    const onTestConnection = vi.fn().mockResolvedValue(false)
    render(
      <Settings
        config={{ ...mockConfig, rpc_servers: [] }}
        onAddServer={vi.fn()}
        onDeleteServer={vi.fn()}
        onTestConnection={onTestConnection}
        onReLogin={vi.fn()}
      />
    )
    await userEvent.type(screen.getByTestId('new-server-url'), 'http://bad-host:6800')
    fireEvent.click(screen.getByTestId('test-connection-button'))
    await waitFor(() => {
      expect(screen.getByTestId('test-connection-result')).toHaveTextContent('连接失败')
    })
  })
})
