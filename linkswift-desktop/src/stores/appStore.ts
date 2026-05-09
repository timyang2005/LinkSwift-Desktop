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

interface AppState {
  config: AppConfig
  shareInfo: ShareInfo | null
  selectedFiles: FileItem[]
  taskQueue: DownloadTask[]
  isLoading: boolean
  error: string | null

  setConfig: (config: AppConfig) => void
  setShareInfo: (info: ShareInfo | null) => void
  toggleFileSelection: (fid: string) => void
  selectAllFiles: () => void
  deselectAllFiles: () => void
  addTask: (task: DownloadTask) => void
  updateTaskStatus: (id: string, status: TaskStatus) => void
  removeTask: (id: string) => void
  setLoading: (loading: boolean) => void
  setError: (error: string | null) => void
  setTheme: (theme: Theme) => void
  addRpcServer: (server: RpcServer) => void
  removeRpcServer: (id: string) => void
  updateRpcServer: (server: RpcServer) => void
  setDefaultRpcServer: (id: string) => void
  setFirstRunComplete: () => void
}

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

export const useAppStore = create<AppState>((set, get) => ({
  config: defaultConfig,
  shareInfo: null,
  selectedFiles: [],
  taskQueue: [],
  isLoading: false,
  error: null,

  setConfig: (config) => set({ config }),
  setShareInfo: (info) => set({ shareInfo: info, selectedFiles: [] }),
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
  selectAllFiles: () => {
    const { shareInfo } = get()
    if (!shareInfo) return
    set({ selectedFiles: [...shareInfo.files] })
  },
  deselectAllFiles: () => set({ selectedFiles: [] }),
  addTask: (task) => set((s) => ({ taskQueue: [...s.taskQueue, task] })),
  updateTaskStatus: (id, status) =>
    set((s) => ({
      taskQueue: s.taskQueue.map((t) => (t.id === id ? { ...t, status } : t)),
    })),
  removeTask: (id) =>
    set((s) => ({ taskQueue: s.taskQueue.filter((t) => t.id !== id) })),
  setLoading: (loading) => set({ isLoading: loading }),
  setError: (error) => set({ error }),
  setTheme: (theme) => set((s) => ({ config: { ...s.config, theme } })),
  addRpcServer: (server) =>
    set((s) => ({
      config: { ...s.config, rpc_servers: [...s.config.rpc_servers, server] },
    })),
  removeRpcServer: (id) =>
    set((s) => ({
      config: {
        ...s.config,
        rpc_servers: s.config.rpc_servers.filter((srv) => srv.id !== id),
      },
    })),
  updateRpcServer: (server) =>
    set((s) => ({
      config: {
        ...s.config,
        rpc_servers: s.config.rpc_servers.map((srv) =>
          srv.id === server.id ? server : srv
        ),
      },
    })),
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
  setFirstRunComplete: () =>
    set((s) => ({ config: { ...s.config, is_first_run: false } })),
}))
