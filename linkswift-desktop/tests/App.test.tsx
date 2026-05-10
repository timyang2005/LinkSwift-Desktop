import { describe, test, expect, beforeEach } from 'vitest'
import { render, screen } from '@testing-library/react'
import App from '../src/App'
import { useAppStore } from '../src/stores/appStore'

describe('App 主组件', () => {
  beforeEach(() => {
    useAppStore.setState({
      config: {
        credential: {
          encrypted_cookie: '',
          last_verified: 0,
          is_valid: false,
          remind_before_expire_days: 7,
        },
        rpc_servers: [],
        default_rpc_index: 0,
        theme: 'System',
        is_first_run: true,
        retry_count: 3,
      },
      shareInfo: null,
      selectedFiles: [],
      taskQueue: [],
      isLoading: false,
      error: null,
    })
  })

  test('首次运行时渲染 FirstRunWizard 而非 Vite 模板', () => {
    useAppStore.setState({
      config: {
        ...useAppStore.getState().config,
        is_first_run: true,
      },
    })
    render(<App />)
    expect(screen.getByTestId('first-run-wizard')).toBeInTheDocument()
    expect(screen.queryByText('Get started')).not.toBeInTheDocument()
  })

  test('非首次运行时渲染主应用界面（LinkInput）', () => {
    useAppStore.setState({
      config: {
        ...useAppStore.getState().config,
        is_first_run: false,
        credential: {
          encrypted_cookie: 'test',
          last_verified: Date.now(),
          is_valid: true,
          remind_before_expire_days: 7,
        },
        rpc_servers: [
          {
            id: 'test-server',
            name: 'Test Aria2',
            url: 'http://localhost:6800',
            downloader_type: 'Aria2',
            is_default: true,
          },
        ],
      },
    })
    render(<App />)
    expect(screen.getByTestId('link-input')).toBeInTheDocument()
    expect(screen.queryByTestId('first-run-wizard')).not.toBeInTheDocument()
    expect(screen.queryByText('Get started')).not.toBeInTheDocument()
  })

  test('不应渲染 Vite 模板的计数器按钮', () => {
    useAppStore.setState({
      config: {
        ...useAppStore.getState().config,
        is_first_run: false,
        credential: {
          encrypted_cookie: 'test',
          last_verified: Date.now(),
          is_valid: true,
          remind_before_expire_days: 7,
        },
        rpc_servers: [
          {
            id: 'test-server',
            name: 'Test Aria2',
            url: 'http://localhost:6800',
            downloader_type: 'Aria2',
            is_default: true,
          },
        ],
      },
    })
    render(<App />)
    const counterButtons = screen.queryAllByRole('button', { name: /count is/i })
    expect(counterButtons).toHaveLength(0)
  })

  test('有分享信息时渲染 FileList', () => {
    useAppStore.setState({
      config: {
        ...useAppStore.getState().config,
        is_first_run: false,
        credential: {
          encrypted_cookie: 'test',
          last_verified: Date.now(),
          is_valid: true,
          remind_before_expire_days: 7,
        },
        rpc_servers: [
          {
            id: 'test-server',
            name: 'Test Aria2',
            url: 'http://localhost:6800',
            downloader_type: 'Aria2',
            is_default: true,
          },
        ],
      },
      shareInfo: {
        pwd_id: 'abc',
        stoken: 'token123',
        title: '测试分享',
        has_password: false,
        files: [
          {
            fid: 'file1',
            name: 'test.txt',
            is_folder: false,
            size: 1024,
            pdir_fid: '',
            selected: false,
          },
        ],
      },
    })
    render(<App />)
    expect(screen.getByTestId('file-list')).toBeInTheDocument()
  })

  test('有任务时渲染 TaskQueue', () => {
    useAppStore.setState({
      config: {
        ...useAppStore.getState().config,
        is_first_run: false,
        credential: {
          encrypted_cookie: 'test',
          last_verified: Date.now(),
          is_valid: true,
          remind_before_expire_days: 7,
        },
        rpc_servers: [
          {
            id: 'test-server',
            name: 'Test Aria2',
            url: 'http://localhost:6800',
            downloader_type: 'Aria2',
            is_default: true,
          },
        ],
      },
      taskQueue: [
        {
          id: 'task1',
          share_url: 'https://pan.quark.cn/s/test',
          files: [],
          status: { type: 'Pending' },
          target_dir: '/downloads',
          rpc_server_id: 'test-server',
          created_at: Date.now(),
          retry_count: 0,
        },
      ],
    })
    render(<App />)
    expect(screen.getByTestId('task-queue')).toBeInTheDocument()
  })
})
