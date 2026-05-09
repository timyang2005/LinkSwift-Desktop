export interface FileItem {
  fid: string
  name: string
  is_folder: boolean
  size: number
  pdir_fid: string
  mime_type?: string
  file_icon?: string
  created_at?: number
  updated_at?: number
  share_fid_token?: string
  selected: boolean
}

export interface PaginatedFiles {
  items: FileItem[]
  total_count: number
  page: number
  page_size: number
  has_more: boolean
}

export interface ShareInfo {
  pwd_id: string
  stoken: string
  title: string
  has_password: boolean
  files: FileItem[]
}

export interface DownloadLink {
  fid: string
  name: string
  url: string
  size: number
  md5?: string
  expires_in: number
}

export interface DownloadTask {
  id: string
  share_url: string
  files: FileItem[]
  status: TaskStatus
  target_dir: string
  rpc_server_id: string
  created_at: number
  error_message?: string
  retry_count: number
}

export type TaskStatus =
  | { type: 'Pending' }
  | { type: 'Parsing' }
  | { type: 'Transferring'; progress: number }
  | { type: 'FetchingLink' }
  | { type: 'Pushing' }
  | { type: 'Completed' }
  | { type: 'Failed'; reason: string }
  | { type: 'Cancelled' }

export interface AppConfig {
  credential: CredentialConfig
  rpc_servers: RpcServer[]
  default_rpc_index: number
  theme: Theme
  proxy?: ProxyConfig
  is_first_run: boolean
  retry_count: number
}

export interface CredentialConfig {
  encrypted_cookie: string
  last_verified: number
  is_valid: boolean
  remind_before_expire_days: number
}

export interface RpcServer {
  id: string
  name: string
  url: string
  token?: string
  downloader_type: DownloaderType
  download_dir?: string
  is_default: boolean
}

export type DownloaderType = 'Aria2' | 'BitComet' | 'ABDownloadManager' | 'Custom'

export interface ProxyConfig {
  url: string
  username?: string
  password?: string
}

export type Theme = 'Light' | 'Dark' | 'System'

export type TransferTaskStatus =
  | { type: 'Pending' }
  | { type: 'Running'; progress: number }
  | { type: 'Completed'; new_fids: string[] }
  | { type: 'Failed'; reason: string }

export type RpcTaskStatus =
  | 'Active'
  | 'Waiting'
  | 'Paused'
  | { type: 'Error'; message: string }
  | 'Complete'
  | 'Removed'
