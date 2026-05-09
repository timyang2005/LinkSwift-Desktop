/**
 * LinkSwift Desktop - 应用状态管理
 * 
 * 使用 Zustand 库管理全局应用状态，包括配置、分享信息、任务队列等
 */

import { create } from 'zustand'
import type {
  ShareInfo,
  FileItem,
  DownloadTask,
  AppConfig,
  TaskStatus,
  Theme,
  RpcServer,
} from '../types'

/**
 * 应用状态接口
 * 定义了所有全局状态和操作方法
 */
interface AppState {
  // 应用配置信息
  config: AppConfig
  // 当前分享链接的解析结果
  shareInfo: ShareInfo | null
  // 用户选中的文件列表
  selectedFiles: FileItem[]
  // 下载任务队列
  taskQueue: DownloadTask[]
  // 是否正在加载数据
  isLoading: boolean
  // 错误信息
  error: string | null

  // 设置应用配置
  setConfig: (config: AppConfig) => void
  // 设置分享信息
  setShareInfo: (info: ShareInfo | null) => void
  // 切换文件选中状态
  toggleFileSelection: (fid: string) => void
  // 全选所有文件
  selectAllFiles: () => void
  // 取消所有文件选中
  deselectAllFiles: () => void
  // 添加下载任务
  addTask: (task: DownloadTask) => void
  // 更新任务状态
  updateTaskStatus: (id: string, status: TaskStatus) => void
  // 移除任务
  removeTask: (id: string) => void
  // 设置加载状态
  setLoading: (loading: boolean) => void
  // 设置错误信息
  setError: (error: string | null) => void
  // 设置主题
  setTheme: (theme: Theme) => void
  // 添加 RPC 服务器
  addRpcServer: (server: RpcServer) => void
  // 移除 RPC 服务器
  removeRpcServer: (id: string) => void
  // 更新 RPC 服务器
  updateRpcServer: (server: RpcServer) => void
  // 设置默认 RPC 服务器
  setDefaultRpcServer: (id: string) => void
  // 设置首次运行完成
  setFirstRunComplete: () => void
}

// 默认配置
const defaultConfig: AppConfig = {
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
}

/**
 * 创建应用状态管理 Hook
 * 使用 Zustand 的 create 方法创建带有预定义状态和操作的状态管理
 */
export const useAppStore = create<AppState>((set, get) => ({
  // 初始化状态
  config: defaultConfig,
  shareInfo: null,
  selectedFiles: [],
  taskQueue: [],
  isLoading: false,
  error: null,

  // 设置配置，同时清空已选文件
  setConfig: (config) => set({ config }),
  setShareInfo: (info) => set({ shareInfo: info, selectedFiles: [] }),

  // 切换文件选中状态：如果文件已选中则取消，否则添加选中
  toggleFileSelection: (fid) => {
    const { shareInfo, selectedFiles } = get()
    if (!shareInfo) return
    const exists = selectedFiles.find((f) => f.fid === fid)
    if (exists) {
      set({ selectedFiles: selectedFiles.filter((f) => f.fid !== fid) })
    } else {
      const file = shareInfo.files.find((f) => f.fid === fid)
      if (file) {
        set({ selectedFiles: [...selectedFiles, file] })
      }
    }
  },

  // 全选所有文件
  selectAllFiles: () => {
    const { shareInfo } = get()
    if (!shareInfo) return
    set({ selectedFiles: [...shareInfo.files] })
  },

  // 取消所有选中
  deselectAllFiles: () => set({ selectedFiles: [] }),

  // 添加新任务到队列
  addTask: (task) => set((s) => ({ taskQueue: [...s.taskQueue, task] })),

  // 更新指定任务的状态
  updateTaskStatus: (id, status) =>
    set((s) => ({
      taskQueue: s.taskQueue.map((t) => (t.id === id ? { ...t, status } : t)),
    })),

  // 从队列中移除任务
  removeTask: (id) =>
    set((s) => ({ taskQueue: s.taskQueue.filter((t) => t.id !== id) })),

  // 设置加载状态
  setLoading: (loading) => set({ isLoading: loading }),

  // 设置错误信息
  setError: (error) => set({ error }),

  // 更新主题设置
  setTheme: (theme) => set((s) => ({ config: { ...s.config, theme } })),

  // 添加新的 RPC 服务器到配置列表
  addRpcServer: (server) =>
    set((s) => ({
      config: { ...s.config, rpc_servers: [...s.config.rpc_servers, server] },
    })),

  // 从配置列表中移除指定 RPC 服务器
  removeRpcServer: (id) =>
    set((s) => ({
      config: {
        ...s.config,
        rpc_servers: s.config.rpc_servers.filter((srv) => srv.id !== id),
      },
    })),

  // 更新已存在的 RPC 服务器信息
  updateRpcServer: (server) =>
    set((s) => ({
      config: {
        ...s.config,
        rpc_servers: s.config.rpc_servers.map((srv) =>
          srv.id === server.id ? server : srv
        ),
      },
    })),

  // 设置默认 RPC 服务器
  setDefaultRpcServer: (id) =>
    set((s) => ({
      config: {
        ...s.config,
        rpc_servers: s.config.rpc_servers.map((srv) => ({
          ...srv,
          is_default: srv.id === id,
        })),
        default_rpc_index: s.config.rpc_servers.findIndex((srv) => srv.id === id),
      },
    })),

  // 标记首次运行完成
  setFirstRunComplete: () =>
    set((s) => ({ config: { ...s.config, is_first_run: false } })),
}))
