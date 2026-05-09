import { describe, test, expect } from 'vitest'
import type {
  FileItem,
  ShareInfo,
  DownloadLink,
  DownloadTask,
  AppConfig,
  CredentialConfig,
  RpcServer,
  DownloaderType,
  ProxyConfig,
  Theme,
  TaskStatus,
  TransferTaskStatus,
  RpcTaskStatus,
  PaginatedFiles,
} from '../../src/types'

describe('FileItem 类型', () => {
  test('FileItem 必填字段完整', () => {
    const item: FileItem = {
      fid: 'abc123',
      name: 'test.mp4',
      is_folder: false,
      size: 1024,
      pdir_fid: '0',
      selected: false,
    }
    expect(item.fid).toBe('abc123')
    expect(item.name).toBe('test.mp4')
    expect(item.is_folder).toBe(false)
    expect(item.size).toBe(1024)
    expect(item.selected).toBe(false)
  })

  test('FileItem 可选字段可省略', () => {
    const item: FileItem = {
      fid: 'f1',
      name: 'doc.txt',
      is_folder: false,
      size: 256,
      pdir_fid: '0',
      selected: false,
    }
    expect(item.mime_type).toBeUndefined()
    expect(item.share_fid_token).toBeUndefined()
  })

  test('FileItem 文件夹类型', () => {
    const folder: FileItem = {
      fid: 'dir1',
      name: '文档',
      is_folder: true,
      size: 0,
      pdir_fid: '0',
      selected: false,
    }
    expect(folder.is_folder).toBe(true)
    expect(folder.size).toBe(0)
  })

  test('FileItem 完整字段', () => {
    const item: FileItem = {
      fid: 'f1',
      name: 'video.mp4',
      is_folder: false,
      size: 1073741824,
      pdir_fid: 'parent1',
      mime_type: 'video/mp4',
      file_icon: 'video',
      created_at: 1700000000,
      updated_at: 1700000100,
      share_fid_token: 'tok123',
      selected: true,
    }
    expect(item.mime_type).toBe('video/mp4')
    expect(item.share_fid_token).toBe('tok123')
    expect(item.selected).toBe(true)
  })
})

describe('ShareInfo 类型', () => {
  test('ShareInfo 无密码分享', () => {
    const info: ShareInfo = {
      pwd_id: 'abc123',
      stoken: 'stoken_val',
      title: '测试分享',
      has_password: false,
      files: [],
    }
    expect(info.has_password).toBe(false)
    expect(info.files).toHaveLength(0)
  })

  test('ShareInfo 有密码分享', () => {
    const info: ShareInfo = {
      pwd_id: 'xyz789',
      stoken: 'tok2',
      title: '加密分享',
      has_password: true,
      files: [
        {
          fid: 'f1',
          name: 'file.mp4',
          is_folder: false,
          size: 1024,
          pdir_fid: '0',
          selected: false,
        },
      ],
    }
    expect(info.has_password).toBe(true)
    expect(info.files).toHaveLength(1)
  })
})

describe('DownloadLink 类型', () => {
  test('DownloadLink 包含直链信息', () => {
    const link: DownloadLink = {
      fid: 'fid1',
      name: 'movie.mp4',
      url: 'https://dl.quark.cn/xxx',
      size: 1073741824,
      md5: 'abc123',
      expires_in: 3600,
    }
    expect(link.url).toContain('https://')
    expect(link.expires_in).toBe(3600)
  })

  test('DownloadLink md5 可选', () => {
    const link: DownloadLink = {
      fid: 'fid2',
      name: 'doc.pdf',
      url: 'https://dl.quark.cn/yyy',
      size: 2048,
      expires_in: 1800,
    }
    expect(link.md5).toBeUndefined()
  })
})

describe('AppConfig 类型', () => {
  test('AppConfig 默认值', () => {
    const config: AppConfig = {
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
    expect(config.is_first_run).toBe(true)
    expect(config.rpc_servers).toHaveLength(0)
    expect(config.retry_count).toBe(3)
  })

  test('AppConfig 完整配置', () => {
    const config: AppConfig = {
      credential: {
        encrypted_cookie: 'enc_data',
        last_verified: 1700000000,
        is_valid: true,
        remind_before_expire_days: 5,
      },
      rpc_servers: [
        {
          id: 'srv1',
          name: 'Aria2',
          url: 'http://localhost:6800',
          downloader_type: 'Aria2',
          is_default: true,
        },
      ],
      default_rpc_index: 0,
      theme: 'Dark',
      proxy: {
        url: 'http://127.0.0.1:7890',
        username: 'user',
        password: 'pass',
      },
      is_first_run: false,
      retry_count: 5,
    }
    expect(config.credential.is_valid).toBe(true)
    expect(config.rpc_servers).toHaveLength(1)
    expect(config.theme).toBe('Dark')
    expect(config.proxy?.url).toBe('http://127.0.0.1:7890')
  })
})

describe('RpcServer 类型', () => {
  test('RpcServer 基本配置', () => {
    const server: RpcServer = {
      id: 'srv1',
      name: 'Aria2',
      url: 'http://localhost:6800',
      downloader_type: 'Aria2',
      is_default: true,
    }
    expect(server.token).toBeUndefined()
    expect(server.download_dir).toBeUndefined()
  })

  test('DownloaderType 所有变体', () => {
    const types: DownloaderType[] = ['Aria2', 'BitComet', 'ABDownloadManager', 'Custom']
    expect(types).toHaveLength(4)
  })
})

describe('TaskStatus 类型', () => {
  test('TaskStatus Pending', () => {
    const status: TaskStatus = { type: 'Pending' }
    expect(status.type).toBe('Pending')
  })

  test('TaskStatus Transferring with progress', () => {
    const status: TaskStatus = { type: 'Transferring', progress: 67 }
    expect(status.type).toBe('Transferring')
    if (status.type === 'Transferring') {
      expect(status.progress).toBe(67)
    }
  })

  test('TaskStatus Failed with reason', () => {
    const status: TaskStatus = { type: 'Failed', reason: 'Cookie expired' }
    expect(status.type).toBe('Failed')
    if (status.type === 'Failed') {
      expect(status.reason).toBe('Cookie expired')
    }
  })
})

describe('PaginatedFiles 类型', () => {
  test('PaginatedFiles 分页信息', () => {
    const pag: PaginatedFiles = {
      items: [],
      total_count: 100,
      page: 1,
      page_size: 50,
      has_more: true,
    }
    expect(pag.has_more).toBe(true)
    expect(pag.total_count).toBe(100)
  })
})

describe('DownloadTask 类型', () => {
  test('DownloadTask 完整结构', () => {
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
    expect(task.id).toBe('task-001')
    expect(task.status.type).toBe('Pending')
  })
})

describe('Theme 类型', () => {
  test('Theme 所有变体', () => {
    const themes: Theme[] = ['Light', 'Dark', 'System']
    expect(themes).toHaveLength(3)
  })
})

describe('ProxyConfig 类型', () => {
  test('ProxyConfig 无认证', () => {
    const proxy: ProxyConfig = { url: 'http://proxy:8080' }
    expect(proxy.username).toBeUndefined()
    expect(proxy.password).toBeUndefined()
  })

  test('ProxyConfig 有认证', () => {
    const proxy: ProxyConfig = {
      url: 'http://proxy:8080',
      username: 'admin',
      password: 'secret',
    }
    expect(proxy.username).toBe('admin')
  })
})
