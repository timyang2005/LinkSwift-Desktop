/**
 * LinkSwift Desktop - TypeScript 类型定义
 * 
 * 定义了应用中使用的所有数据结构和类型
 */

// 文件项接口，表示分享链接中的一个文件或文件夹
export interface FileItem {
  fid: string                        // 文件唯一标识符
  name: string                      // 文件名
  is_folder: boolean                 // 是否为文件夹
  size: number                      // 文件大小（字节）
  pdir_fid: string                   // 父目录 ID
  mime_type?: string                 // MIME 类型
  file_icon?: string                // 文件图标
  created_at?: number               // 创建时间戳
  updated_at?: number               // 更新时间戳
  share_fid_token?: string          // 分享标识符
  selected: boolean                 // 是否被选中
}

// 分页文件列表接口
export interface PaginatedFiles {
  items: FileItem[]                  // 文件列表
  total_count: number                // 总数量
  page: number                       // 当前页码
  page_size: number                  // 每页数量
  has_more: boolean                  // 是否有更多
}

// 分享信息接口，表示解析后的分享链接内容
export interface ShareInfo {
  pwd_id: string                    // 密码 ID（如果有密码）
  stoken: string                     // 安全令牌
  title: string                      // 分享标题
  has_password: boolean              // 是否有密码保护
  files: FileItem[]                  // 文件列表
}

// 下载直链接口，表示从网盘获取的下载链接
export interface DownloadLink {
  fid: string                        // 文件 ID
  name: string                       // 文件名
  url: string                        // 直链 URL
  size: number                       // 文件大小
  md5?: string                       // MD5 校验值
  expires_in: number                 // 链接过期时间（秒）
}

// 下载任务接口，表示一个下载任务
export interface DownloadTask {
  id: string                         // 任务唯一 ID
  share_url: string                  // 来源分享链接
  files: FileItem[]                  // 要下载的文件列表
  status: TaskStatus                 // 当前状态
  target_dir: string                 // 目标下载目录
  rpc_server_id: string              // 使用的 RPC 服务器 ID
  created_at: number                 // 创建时间戳
  error_message?: string             // 错误信息（如果有）
  retry_count: number                // 重试次数
}

// 任务状态类型，表示下载任务可能处于的各种状态
export type TaskStatus =
  | { type: 'Pending' }              // 等待中
  | { type: 'Parsing' }              // 解析中
  | { type: 'Transferring'; progress: number }  // 转存中（带进度）
  | { type: 'FetchingLink' }         // 获取直链中
  | { type: 'Pushing' }              // 推送到下载器中
  | { type: 'Completed' }            // 已完成
  | { type: 'Failed'; reason: string }  // 失败（带原因）
  | { type: 'Cancelled' }           // 已取消

// 应用配置接口，存储用户的所有配置
export interface AppConfig {
  credential: CredentialConfig        // 登录凭证配置
  rpc_servers: RpcServer[]           // RPC 服务器列表
  default_rpc_index: number           // 默认服务器索引
  theme: Theme                        // 主题设置
  proxy?: ProxyConfig                 // 代理配置
  is_first_run: boolean               // 是否首次运行
  retry_count: number                // 重试次数
}

// 登录凭证配置
export interface CredentialConfig {
  encrypted_cookie: string            // 加密的 Cookie
  last_verified: number              // 最后验证时间戳
  is_valid: boolean                  // 凭证是否有效
  remind_before_expire_days: number  // 过期前提醒天数
}

// RPC 服务器配置
export interface RpcServer {
  id: string                         // 服务器唯一 ID
  name: string                       // 服务器名称
  url: string                        // RPC 地址
  token?: string                     // RPC 密钥（可选）
  downloader_type: DownloaderType    // 下载器类型
  download_dir?: string              // 下载目录（可选）
  is_default: boolean                // 是否为默认服务器
}

// 下载器类型枚举
export type DownloaderType = 'Aria2' | 'BitComet' | 'ABDownloadManager' | 'Custom'

// 代理配置
export interface ProxyConfig {
  url: string                        // 代理地址
  username?: string                  // 用户名（可选）
  password?: string                  // 密码（可选）
}

// 主题类型
export type Theme = 'Light' | 'Dark' | 'System'

// 转存任务状态
export type TransferTaskStatus =
  | { type: 'Pending' }              // 等待中
  | { type: 'Running'; progress: number }  // 运行中（带进度）
  | { type: 'Completed'; new_fids: string[] }  // 完成（返回新文件 ID）
  | { type: 'Failed'; reason: string }  // 失败（带原因）

// RPC 任务状态
export type RpcTaskStatus =
  | 'Active'                         // 活动中
  | 'Waiting'                        // 等待中
  | 'Paused'                         // 已暂停
  | { type: 'Error'; message: string }  // 错误
  | 'Complete'                       // 已完成
  | 'Removed'                        // 已移除
