import { describe, test, expect, beforeEach } from 'vitest'
import { useAppStore } from '../src/stores/appStore'
import type { ShareInfo, FileItem, DownloadTask, RpcServer } from '../src/types'

const mockFileItem: FileItem = {
  fid: 'f1',
  name: 'test.mp4',
  is_folder: false,
  size: 1024,
  pdir_fid: '0',
  selected: false,
}

const mockShareInfo: ShareInfo = {
  pwd_id: 'abc123',
  stoken: 'stoken_val',
  title: '测试分享',
  has_password: false,
  files: [
    mockFileItem,
    {
      fid: 'f2',
      name: 'doc.txt',
      is_folder: false,
      size: 256,
      pdir_fid: '0',
      selected: false,
    },
    {
      fid: 'dir1',
      name: '文件夹',
      is_folder: true,
      size: 0,
      pdir_fid: '0',
      selected: false,
    },
  ],
}

const mockRpcServer: RpcServer = {
  id: 'srv1',
  name: 'Aria2',
  url: 'http://localhost:6800',
  downloader_type: 'Aria2',
  is_default: true,
}

describe('useAppStore', () => {
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

  test('初始状态: isFirstRun 为 true', () => {
    const state = useAppStore.getState()
    expect(state.config.is_first_run).toBe(true)
  })

  test('初始状态: shareInfo 为 null', () => {
    const state = useAppStore.getState()
    expect(state.shareInfo).toBeNull()
  })

  test('初始状态: rpc_servers 为空', () => {
    const state = useAppStore.getState()
    expect(state.config.rpc_servers).toHaveLength(0)
  })

  test('setShareInfo 更新分享信息', () => {
    useAppStore.getState().setShareInfo(mockShareInfo)
    const state = useAppStore.getState()
    expect(state.shareInfo).toEqual(mockShareInfo)
    expect(state.selectedFiles).toHaveLength(0)
  })

  test('setShareInfo 为 null 时清空选中文件', () => {
    useAppStore.getState().setShareInfo(mockShareInfo)
    useAppStore.getState().toggleFileSelection('f1')
    useAppStore.getState().setShareInfo(null)
    const state = useAppStore.getState()
    expect(state.shareInfo).toBeNull()
    expect(state.selectedFiles).toHaveLength(0)
  })

  test('toggleFileSelection 选中文件', () => {
    useAppStore.getState().setShareInfo(mockShareInfo)
    useAppStore.getState().toggleFileSelection('f1')
    const state = useAppStore.getState()
    expect(state.selectedFiles).toHaveLength(1)
    expect(state.selectedFiles[0].fid).toBe('f1')
  })

  test('toggleFileSelection 取消选中', () => {
    useAppStore.getState().setShareInfo(mockShareInfo)
    useAppStore.getState().toggleFileSelection('f1')
    useAppStore.getState().toggleFileSelection('f1')
    const state = useAppStore.getState()
    expect(state.selectedFiles).toHaveLength(0)
  })

  test('toggleFileSelection 多次选中不同文件', () => {
    useAppStore.getState().setShareInfo(mockShareInfo)
    useAppStore.getState().toggleFileSelection('f1')
    useAppStore.getState().toggleFileSelection('f2')
    const state = useAppStore.getState()
    expect(state.selectedFiles).toHaveLength(2)
  })

  test('selectAllFiles 全选', () => {
    useAppStore.getState().setShareInfo(mockShareInfo)
    useAppStore.getState().selectAllFiles()
    const state = useAppStore.getState()
    expect(state.selectedFiles).toHaveLength(3)
  })

  test('deselectAllFiles 全部取消', () => {
    useAppStore.getState().setShareInfo(mockShareInfo)
    useAppStore.getState().selectAllFiles()
    useAppStore.getState().deselectAllFiles()
    const state = useAppStore.getState()
    expect(state.selectedFiles).toHaveLength(0)
  })

  test('toggleFileSelection 无 shareInfo 时不操作', () => {
    useAppStore.getState().toggleFileSelection('f1')
    expect(useAppStore.getState().selectedFiles).toHaveLength(0)
  })

  test('addTask 添加任务', () => {
    const task: DownloadTask = {
      id: 'task-001',
      share_url: 'https://pan.quark.cn/s/abc',
      files: [mockFileItem],
      status: { type: 'Pending' },
      target_dir: '0',
      rpc_server_id: 'srv1',
      created_at: 1700000000,
      retry_count: 0,
    }
    useAppStore.getState().addTask(task)
    const state = useAppStore.getState()
    expect(state.taskQueue).toHaveLength(1)
    expect(state.taskQueue[0].id).toBe('task-001')
  })

  test('updateTaskStatus 更新任务状态', () => {
    const task: DownloadTask = {
      id: 'task-001',
      share_url: 'https://pan.quark.cn/s/abc',
      files: [],
      status: { type: 'Pending' },
      target_dir: '0',
      rpc_server_id: 'srv1',
      created_at: 1700000000,
      retry_count: 0,
    }
    useAppStore.getState().addTask(task)
    useAppStore.getState().updateTaskStatus('task-001', { type: 'Transferring', progress: 50 })
    const state = useAppStore.getState()
    expect(state.taskQueue[0].status.type).toBe('Transferring')
  })

  test('updateTaskStatus 不影响其他任务', () => {
    const task1: DownloadTask = {
      id: 'task-001',
      share_url: 'https://pan.quark.cn/s/abc',
      files: [],
      status: { type: 'Pending' },
      target_dir: '0',
      rpc_server_id: 'srv1',
      created_at: 1700000000,
      retry_count: 0,
    }
    const task2: DownloadTask = {
      id: 'task-002',
      share_url: 'https://pan.quark.cn/s/xyz',
      files: [],
      status: { type: 'Pending' },
      target_dir: '0',
      rpc_server_id: 'srv1',
      created_at: 1700000000,
      retry_count: 0,
    }
    useAppStore.getState().addTask(task1)
    useAppStore.getState().addTask(task2)
    useAppStore.getState().updateTaskStatus('task-001', { type: 'Completed' })
    const state = useAppStore.getState()
    expect(state.taskQueue[0].status.type).toBe('Completed')
    expect(state.taskQueue[1].status.type).toBe('Pending')
  })

  test('removeTask 删除任务', () => {
    const task: DownloadTask = {
      id: 'task-001',
      share_url: 'https://pan.quark.cn/s/abc',
      files: [],
      status: { type: 'Pending' },
      target_dir: '0',
      rpc_server_id: 'srv1',
      created_at: 1700000000,
      retry_count: 0,
    }
    useAppStore.getState().addTask(task)
    useAppStore.getState().removeTask('task-001')
    expect(useAppStore.getState().taskQueue).toHaveLength(0)
  })

  test('setLoading 设置加载状态', () => {
    useAppStore.getState().setLoading(true)
    expect(useAppStore.getState().isLoading).toBe(true)
    useAppStore.getState().setLoading(false)
    expect(useAppStore.getState().isLoading).toBe(false)
  })

  test('setError 设置错误信息', () => {
    useAppStore.getState().setError('Cookie已失效')
    expect(useAppStore.getState().error).toBe('Cookie已失效')
    useAppStore.getState().setError(null)
    expect(useAppStore.getState().error).toBeNull()
  })

  test('setTheme 切换主题', () => {
    useAppStore.getState().setTheme('Dark')
    expect(useAppStore.getState().config.theme).toBe('Dark')
    useAppStore.getState().setTheme('Light')
    expect(useAppStore.getState().config.theme).toBe('Light')
  })

  test('addRpcServer 添加 RPC 服务器', () => {
    useAppStore.getState().addRpcServer(mockRpcServer)
    const state = useAppStore.getState()
    expect(state.config.rpc_servers).toHaveLength(1)
    expect(state.config.rpc_servers[0].name).toBe('Aria2')
  })

  test('removeRpcServer 删除 RPC 服务器', () => {
    useAppStore.getState().addRpcServer(mockRpcServer)
    useAppStore.getState().removeRpcServer('srv1')
    expect(useAppStore.getState().config.rpc_servers).toHaveLength(0)
  })

  test('updateRpcServer 更新 RPC 服务器', () => {
    useAppStore.getState().addRpcServer(mockRpcServer)
    const updated = { ...mockRpcServer, url: 'http://newhost:6800', token: 'new_token' }
    useAppStore.getState().updateRpcServer(updated)
    const state = useAppStore.getState()
    expect(state.config.rpc_servers[0].url).toBe('http://newhost:6800')
    expect(state.config.rpc_servers[0].token).toBe('new_token')
  })

  test('setDefaultRpcServer 设置默认服务器', () => {
    const server2: RpcServer = {
      id: 'srv2',
      name: 'BitComet',
      url: 'http://localhost:8888',
      downloader_type: 'BitComet',
      is_default: false,
    }
    useAppStore.getState().addRpcServer(mockRpcServer)
    useAppStore.getState().addRpcServer(server2)
    useAppStore.getState().setDefaultRpcServer('srv2')
    const state = useAppStore.getState()
    expect(state.config.rpc_servers[0].is_default).toBe(false)
    expect(state.config.rpc_servers[1].is_default).toBe(true)
  })

  test('setFirstRunComplete 标记首次运行完成', () => {
    expect(useAppStore.getState().config.is_first_run).toBe(true)
    useAppStore.getState().setFirstRunComplete()
    expect(useAppStore.getState().config.is_first_run).toBe(false)
  })

  test('setConfig 完整替换配置', () => {
    const newConfig = {
      credential: {
        encrypted_cookie: 'enc',
        last_verified: 1700000000,
        is_valid: true,
        remind_before_expire_days: 5,
      },
      rpc_servers: [mockRpcServer],
      default_rpc_index: 0,
      theme: 'Dark' as const,
      is_first_run: false,
      retry_count: 5,
    }
    useAppStore.getState().setConfig(newConfig)
    const state = useAppStore.getState()
    expect(state.config.credential.is_valid).toBe(true)
    expect(state.config.theme).toBe('Dark')
    expect(state.config.is_first_run).toBe(false)
  })
})
