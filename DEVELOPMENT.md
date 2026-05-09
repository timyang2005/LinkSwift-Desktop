<div align="center">

# LinkSwift Desktop - 开发者文档

> 面向开发者的技术文档，包含架构设计、开发指南、API 参考和贡献流程。

[← 返回主文档](README.md)

</div>

---

## 目录

- [技术架构](#-技术架构)
- [开发环境搭建](#-开发环境搭建)
- [项目结构](#-项目结构)
- [数据模型](#-数据模型)
- [API 参考](#-api-参考)
- [夸克网盘 API 对接](#-夸克网盘-api-对接)
- [WebView 登录实现](#-webview-登录实现)
- [测试指南](#-测试指南)
- [构建与发布](#-构建与发布)
- [贡献指南](#-贡献指南)

---

## 🏗️ 技术架构

### 整体架构

```
┌─────────────────────────────────────────────────────────────┐
│                    LinkSwift Desktop                         │
│                    (Tauri 2.x)                              │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌─────────────────────────────────────────────────────┐    │
│  │              前端 (React + TypeScript)               │    │
│  │                                                      │    │
│  │  ┌──────────┐ ┌──────────┐ ┌───────────────────┐   │    │
│  │  │  UI组件   │ │  Hooks   │ │  状态管理(Zustand) │   │    │
│  │  │          │ │          │ │                   │   │    │
│  │  │ LinkInput│ │ useQuark │ │  appStore         │   │    │
│  │  │ FileList │ │ useRPC   │ │  - shareInfo      │   │    │
│  │  │ Settings │ │ useConfig│ │  - taskQueue      │   │    │
│  │  │ Wizard   │ │          │ │  - config         │   │    │
│  │  └──────────┘ └──────────┘ └───────────────────┘   │    │
│  └──────────────────────┬──────────────────────────────┘    │
│                         │ Tauri IPC (invoke)                │
│  ┌──────────────────────┴──────────────────────────────┐    │
│  │              后端 (Rust)                             │    │
│  │                                                      │    │
│  │  ┌──────────┐ ┌──────────┐ ┌───────────────────┐   │    │
│  │  │ Commands │ │  Models  │ │   Services        │   │    │
│  │  │          │ │          │ │                   │   │    │
│  │  │ quark.rs │ │ file.rs  │ │  quark_api.rs     │   │    │
│  │  │ rpc.rs   │ │ share.rs │ │  rpc_client.rs    │   │    │
│  │  │ config.rs│ │ config.rs│ │  crypto.rs        │   │    │
│  │  └──────────┘ └──────────┘ └───────────────────┘   │    │
│  └──────────────────────┬──────────────────────────────┘    │
│                         │                                   │
│  ┌──────────────────────┴──────────────────────────────┐    │
│  │              外部服务                                │    │
│  │                                                      │    │
│  │  ┌──────────┐ ┌──────────┐ ┌───────────────────┐   │    │
│  │  │夸克网盘   │ │ RPC下载器 │ │  本地存储         │   │    │
│  │  │ API      │ │ Aria2等  │ │  SQLite/DPAPI    │   │    │
│  │  └──────────┘ └──────────┘ └───────────────────┘   │    │
│  └─────────────────────────────────────────────────────┘    │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

### 技术选型

| 组件 | 选择 | 版本 | 理由 |
|------|------|------|------|
| **桌面框架** | Tauri | 2.x | 轻量（~5MB）、安全、Rust 后端 |
| **前端框架** | React | 19.x | 生态成熟、组件丰富 |
| **类型系统** | TypeScript | 5.x | 类型安全、减少运行时错误 |
| **UI 组件** | shadcn/ui | latest | 可定制、无样式锁定 |
| **CSS 方案** | TailwindCSS | 4.x | 原子化 CSS、快速开发 |
| **状态管理** | Zustand | 5.x | 轻量、简洁、支持持久化 |
| **HTTP 客户端** | reqwest | 0.12.x | Rust 高性能 HTTP 客户端 |
| **本地数据库** | SQLite | via rusqlite | 轻量、无需额外服务 |
| **序列化** | serde | 1.x | Rust 标准序列化框架 |
| **加密** | Windows DPAPI | - | 系统级凭证加密 |

---

## 🔧 开发环境搭建

### 前置要求

| 工具 | 最低版本 | 安装方式 |
|------|---------|---------|
| **Rust** | 1.75+ | [rustup.rs](https://rustup.rs/) |
| **Node.js** | 20+ | [nodejs.org](https://nodejs.org/) |
| **pnpm** | 9+ | `npm install -g pnpm` |
| **WebView2** | 最新 | Win10/11 通常已内置 |
| **Visual Studio Build Tools** | 2022 | [下载](https://visualstudio.microsoft.com/visual-cpp-build-tools/) |

### 安装步骤

```bash
# 1. 克隆仓库
git clone https://github.com/your-username/LinkSwift-Desktop.git
cd LinkSwift-Desktop

# 2. 安装前端依赖
pnpm install

# 3. 开发模式运行（热重载）
pnpm tauri dev

# 4. 运行测试
pnpm test

# 5. 运行 Rust 测试
cd src-tauri && cargo test
```

### VS Code 推荐

安装以下扩展以获得最佳开发体验：

- **rust-analyzer** - Rust 语言支持
- **Tauri** - Tauri 命令集成
- **Tailwind CSS IntelliSense** - TailwindCSS 自动补全
- **ESLint** - JavaScript/TypeScript 代码检查

---

## 📁 项目结构

```
linkswift-desktop/
├── src/                          # React 前端源码
│   ├── components/               # UI 组件
│   │   ├── ui/                   # 基础 UI 组件 (shadcn/ui)
│   │   ├── LinkInput.tsx        # 链接输入组件
│   │   ├── FileList.tsx         # 文件列表组件
│   │   ├── FileItem.tsx         # 文件项组件
│   │   ├── TaskQueue.tsx        # 任务队列组件
│   │   ├── Settings.tsx         # 设置面板
│   │   ├── FirstRunWizard.tsx   # 首次运行向导
│   │   └── TrayMenu.tsx         # 系统托盘菜单
│   ├── hooks/                   # React Hooks
│   │   ├── useQuark.ts          # 夸克网盘 API Hook
│   │   ├── useRPC.ts            # RPC 下载器 Hook
│   │   ├── useConfig.ts         # 配置管理 Hook
│   │   └── useTheme.ts          # 主题切换 Hook
│   ├── stores/                  # Zustand 状态管理
│   │   └── appStore.ts          # 全局应用状态
│   ├── lib/                     # 工具库
│   │   ├── utils.ts             # 通用工具函数
│   │   ├── format.ts            # 格式化工具（文件大小、时间）
│   │   └── constants.ts         # 常量定义
│   ├── types/                   # TypeScript 类型定义
│   │   └── index.ts             # 前端类型
│   ├── App.tsx                  # 主应用组件
│   └── main.tsx                 # 入口文件
│
├── src-tauri/                   # Rust 后端源码
│   ├── src/
│   │   ├── main.rs              # Tauri 入口
│   │   ├── lib.rs               # 库导出
│   │   ├── commands/            # Tauri 命令（前端可调用）
│   │   │   ├── mod.rs
│   │   │   ├── quark.rs         # 夸克网盘相关命令
│   │   │   ├── rpc.rs           # RPC 下载器命令
│   │   │   ├── config.rs        # 配置管理命令
│   │   │   └── auth.rs          # WebView 登录命令
│   │   ├── services/            # 业务逻辑层
│   │   │   ├── mod.rs
│   │   │   ├── quark_api.rs     # 夸克网盘 API 封装
│   │   │   ├── rpc_client.rs    # RPC 客户端封装
│   │   │   ├── crypto.rs        # 加密/解密服务
│   │   │   └── task_queue.rs    # 任务队列管理
│   │   └── models/              # 数据模型
│   │       ├── mod.rs
│   │       ├── file.rs          # 文件/文件夹模型
│   │       ├── share.rs         # 分享信息模型
│   │       ├── config.rs        # 配置模型
│   │       └── task.rs          # 任务模型
│   ├── Cargo.toml               # Rust 依赖配置
│   └── tauri.conf.json          # Tauri 配置
│
├── tests/                       # 测试文件
│   ├── quark.test.ts            # 夸克 API 测试
│   ├── rpc.test.ts              # RPC 测试
│   ├── config.test.ts           # 配置管理测试
│   └── integration.test.ts      # 集成测试
│
├── docs/                        # 文档
│   ├── SPEC.md                  # 产品设计规范
│   └── API.md                   # API 详细文档
│
├── package.json
├── tsconfig.json
├── vite.config.ts
├── tailwind.config.ts
├── README.md                    # 用户文档
└── DEVELOPMENT.md               # 本文档
```

---

## 📊 数据模型

### 配置模型 (Rust)

```rust
/// 应用全局配置
#[derive(Serialize, Deserialize, Clone)]
pub struct AppConfig {
    /// 夸克网盘登录凭证
    pub credential: CredentialConfig,
    /// RPC 服务器列表
    pub rpc_servers: Vec<RpcServer>,
    /// 默认 RPC 服务器索引
    pub default_rpc_index: usize,
    /// 主题设置
    pub theme: Theme,
    /// 代理设置
    pub proxy: Option<ProxyConfig>,
    /// 是否首次运行
    pub is_first_run: bool,
    /// 自动重试次数
    pub retry_count: u32,
}

/// 登录凭证配置（使用 WebView 登录后自动填充）
#[derive(Serialize, Deserialize, Clone)]
pub struct CredentialConfig {
    /// 加密存储的 Cookie
    pub encrypted_cookie: String,
    /// 最后验证时间戳
    pub last_verified: i64,
    /// 凭证是否有效
    pub is_valid: bool,
    /// 过期提醒天数
    pub remind_before_expire_days: u32,
}

/// RPC 服务器配置
#[derive(Serialize, Deserialize, Clone)]
pub struct RpcServer {
    /// 唯一标识
    pub id: String,
    /// 服务器名称
    pub name: String,
    /// 服务器地址
    pub url: String,
    /// RPC 密钥
    pub token: Option<String>,
    /// 下载器类型
    pub downloader_type: DownloaderType,
    /// 默认下载目录
    pub download_dir: Option<String>,
    /// 是否为默认服务器
    pub is_default: bool,
}

/// 下载器类型
#[derive(Serialize, Deserialize, Clone)]
pub enum DownloaderType {
    Aria2,
    BitComet,
    ABDownloadManager,
    Custom,
}

/// 代理配置
#[derive(Serialize, Deserialize, Clone)]
pub struct ProxyConfig {
    /// 代理地址
    pub url: String,
    /// 用户名（可选）
    pub username: Option<String>,
    /// 密码（可选）
    pub password: Option<String>,
}

/// 主题设置
#[derive(Serialize, Deserialize, Clone)]
pub enum Theme {
    Light,
    Dark,
    System,
}
```

### 文件模型 (Rust)

```rust
/// 文件/文件夹项
#[derive(Serialize, Deserialize, Clone)]
pub struct FileItem {
    /// 文件 ID
    pub fid: String,
    /// 文件名
    pub name: String,
    /// 是否为文件夹
    pub is_folder: bool,
    /// 文件大小（字节）
    pub size: u64,
    /// MIME 类型
    pub mime_type: Option<String>,
    /// 文件格式图标标识
    pub file_icon: Option<String>,
    /// 创建时间
    pub created_at: Option<i64>,
    /// 修改时间
    pub updated_at: Option<i64>,
    /// 分享文件的 Token（转存时需要）
    pub share_fid_token: Option<String>,
}

/// 分享信息
#[derive(Serialize, Deserialize, Clone)]
pub struct ShareInfo {
    /// 分享链接 ID
    pub pwd_id: String,
    /// 分享 Token
    pub stoken: String,
    /// 分享标题
    pub title: String,
    /// 是否需要密码
    pub has_password: bool,
    /// 根目录文件列表
    pub files: Vec<FileItem>,
}

/// 下载直链
#[derive(Serialize, Deserialize, Clone)]
pub struct DownloadLink {
    /// 文件 ID
    pub fid: String,
    /// 文件名
    pub name: String,
    /// 直链 URL
    pub url: String,
    /// 文件大小
    pub size: u64,
    /// MD5 校验
    pub md5: Option<String>,
    /// 直链有效期（秒）
    pub expires_in: u32,
}
```

### 任务模型 (Rust)

```rust
/// 下载任务
#[derive(Serialize, Deserialize, Clone)]
pub struct DownloadTask {
    /// 任务唯一 ID
    pub id: String,
    /// 分享链接
    pub share_url: String,
    /// 选中的文件列表
    pub files: Vec<FileItem>,
    /// 任务状态
    pub status: TaskStatus,
    /// 转存目标目录
    pub target_dir: String,
    /// RPC 服务器 ID
    pub rpc_server_id: String,
    /// 创建时间
    pub created_at: i64,
    /// 错误信息
    pub error_message: Option<String>,
    /// 重试次数
    pub retry_count: u32,
}

/// 任务状态
#[derive(Serialize, Deserialize, Clone)]
pub enum TaskStatus {
    /// 等待处理
    Pending,
    /// 正在解析链接
    Parsing,
    /// 正在转存
    Transferring { progress: u32 },
    /// 正在获取直链
    FetchingLink,
    /// 正在推送 RPC
    Pushing,
    /// 已完成
    Completed,
    /// 失败
    Failed { reason: String },
    /// 已取消
    Cancelled,
}
```

---

## 🔌 API 参考

### Tauri 命令（前端 → 后端）

#### 夸克网盘命令

```rust
// 验证登录凭证是否有效
#[tauri::command]
async fn verify_credential() -> Result<CredentialStatus, String>

// 打开 WebView 登录窗口
#[tauri::command]
async fn open_login_window() -> Result<(), String>

// 解析分享链接
#[tauri::command]
async fn parse_share_link(share_url: String) -> Result<ShareInfo, String>

// 提交分享密码
#[tauri::command]
async fn submit_share_password(
    pwd_id: String,
    stoken: String,
    password: String,
) -> Result<ShareInfo, String>

// 获取子目录文件列表
#[tauri::command]
async fn get_share_files(
    pwd_id: String,
    stoken: String,
    dir_fid: String,
    page: u32,
    size: u32,
) -> Result<PaginatedFiles, String>

// 获取用户网盘目录列表（选择转存目标）
#[tauri::command]
async fn get_user_directories(parent_fid: String) -> Result<Vec<FileItem>, String>

// 转存文件到用户网盘
#[tauri::command]
async fn transfer_files(
    pwd_id: String,
    stoken: String,
    fid_list: Vec<String>,
    fid_token_list: Vec<String>,
    target_dir_fid: String,
) -> Result<TaskId, String>

// 查询转存任务状态
#[tauri::command]
async fn query_transfer_task(task_id: String) -> Result<TransferTaskStatus, String>

// 获取文件下载直链
#[tauri::command]
async fn get_download_link(fid: String) -> Result<DownloadLink, String>
```

#### RPC 下载器命令

```rust
// 添加下载任务
#[tauri::command]
async fn add_download_task(
    rpc_server_id: String,
    urls: Vec<String>,
    options: DownloadOptions,
) -> Result<Vec<RpcTaskId>, String>

// 测试 RPC 连接
#[tauri::command]
async fn test_rpc_connection(
    url: String,
    token: Option<String>,
) -> Result<bool, String>

// 查询 RPC 任务状态
#[tauri::command]
async fn query_rpc_task_status(
    rpc_server_id: String,
    task_id: String,
) -> Result<RpcTaskStatus, String>
```

#### 配置管理命令

```rust
// 获取完整配置
#[tauri::command]
async fn get_config() -> Result<AppConfig, String>

// 保存完整配置
#[tauri::command]
async fn save_config(config: AppConfig) -> Result<(), String>

// 添加 RPC 服务器
#[tauri::command]
async fn add_rpc_server(server: RpcServer) -> Result<(), String>

// 更新 RPC 服务器
#[tauri::command]
async fn update_rpc_server(server: RpcServer) -> Result<(), String>

// 删除 RPC 服务器
#[tauri::command]
async fn delete_rpc_server(server_id: String) -> Result<(), String>

// 设置默认 RPC 服务器
#[tauri::command]
async fn set_default_rpc_server(server_id: String) -> Result<(), String>

// 获取下载历史
#[tauri::command]
async fn get_download_history(limit: u32, offset: u32) -> Result<Vec<DownloadTask>, String>

// 清空下载历史
#[tauri::command]
async fn clear_download_history() -> Result<(), String>
```

### 前端调用示例

```typescript
import { invoke } from '@tauri-apps/api/core';

// 解析分享链接
const shareInfo = await invoke<ShareInfo>('parse_share_link', {
  shareUrl: 'https://pan.quark.cn/s/abc123',
});

// 转存文件
const taskId = await invoke<string>('transfer_files', {
  pwdId: shareInfo.pwd_id,
  stoken: shareInfo.stoken,
  fidList: selectedFiles.map(f => f.fid),
  fidTokenList: selectedFiles.map(f => f.share_fid_token),
  targetDirFid: '0', // 根目录
});

// 获取直链
const link = await invoke<DownloadLink>('get_download_link', {
  fid: savedFileId,
});

// 推送到 RPC
const rpcTaskIds = await invoke<string[]>('add_download_task', {
  rpcServerId: 'aria2-main',
  urls: [link.url],
  options: {
    filename: link.name,
    dir: 'D:\\Downloads',
  },
});
```

---

## 🌐 夸克网盘 API 对接

### API 端点

| 功能 | 端点 | 方法 |
|------|------|------|
| 获取分享 Token | `/1/clouddrive/share/sharepage/token` | POST |
| 获取分享文件列表 | `/1/clouddrive/share/sharepage/detail` | GET |
| 转存文件 | `/1/clouddrive/share/sharepage/save` | POST |
| 查询任务状态 | `/1/clouddrive/task` | GET |
| 获取下载直链 | `/1/clouddrive/file/download` | POST |
| 获取用户文件列表 | `/1/clouddrive/file/sort` | GET |

### 基础域名

```
https://drive-pc.quark.cn
```

### 请求头

```rust
const HEADERS: &[(&str, &str)] = &[
    ("Content-Type", "application/json"),
    ("Accept", "application/json, text/plain, */*"),
    ("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36"),
    ("Referer", "https://pan.quark.cn/"),
];
```

### 核心流程

```
1. 获取分享 Token
   POST /1/clouddrive/share/sharepage/token
   Body: { "pwd_id": "分享ID", "passcode": "密码(可选)" }
   → 返回 stoken

2. 获取文件列表
   GET /1/clouddrive/share/sharepage/detail?pwd_id=xxx&stoken=xxx&pdir_fid=0
   → 返回文件列表

3. 转存文件
   POST /1/clouddrive/share/sharepage/save
   Body: {
     "fid_list": ["文件ID"],
     "fid_token_list": ["文件Token"],
     "to_pdir_fid": "目标目录ID",
     "pwd_id": "分享ID",
     "stoken": "分享Token"
   }
   → 返回 task_id

4. 轮询任务状态
   GET /1/clouddrive/task?task_id=xxx
   → 返回转存结果（包含新文件ID）

5. 获取下载直链
   POST /1/clouddrive/file/download
   Body: { "fids": ["文件ID"] }
   → 返回下载直链
```

### 错误码

| HTTP 状态码 | 含义 | 处理方式 |
|------------|------|---------|
| 200 | 成功 | 正常处理 |
| 400 | 参数错误 | 检查请求参数 |
| 401 | 未授权 | Cookie 过期，重新登录 |
| 403 | 无权限 | 分享链接已失效 |
| 404 | 不存在 | 文件已被删除 |
| 429 | 请求频繁 | 退避重试 |
| 500 | 服务器错误 | 稍后重试 |

---

## 🔐 WebView 登录实现

### 实现原理

```
┌──────────────────────────────────────────────────────┐
│                    登录流程                            │
├──────────────────────────────────────────────────────┤
│                                                       │
│  1. 创建 WebView 窗口                                 │
│     ├── 加载 https://pan.quark.cn                     │
│     └── 窗口大小: 800x600                             │
│                                                       │
│  2. 用户在窗口中登录                                   │
│     ├── 扫码 / 密码 / 验证码                          │
│     └── 登录成功后页面 URL 发生变化                    │
│                                                       │
│  3. 检测登录成功                                       │
│     ├── 方式A: 监听 URL 变化                          │
│     ├── 方式B: 注入 JS 检测页面元素                   │
│     └── 方式C: 轮询 Cookie 变化                       │
│                                                       │
│  4. 提取 Cookie                                       │
│     ├── 通过 WebView2 Cookie Manager API              │
│     └── 或注入 document.cookie                        │
│                                                       │
│  5. 加密存储并关闭窗口                                 │
│     ├── 使用 DPAPI 加密 Cookie                        │
│     ├── 保存到本地配置                                 │
│     └── 关闭登录窗口                                  │
│                                                       │
└──────────────────────────────────────────────────────┘
```

### Rust 代码骨架

```rust
use tauri::{WebviewUrl, WebviewWindowBuilder};

/// 打开夸克网盘登录窗口
pub async fn open_login_window(app: tauri::AppHandle) -> Result<String, String> {
    let window = WebviewWindowBuilder::new(
        &app,
        "quark-login",
        WebviewUrl::External("https://pan.quark.cn".parse().unwrap()),
    )
    .title("登录夸克网盘")
    .inner_size(800.0, 600.0)
    .resizable(true)
    .build()
    .map_err(|e| e.to_string())?;

    // 等待登录成功（通过事件监听）
    let cookie = wait_for_login(&window).await?;

    // 加密存储 Cookie
    let encrypted = crypto::encrypt_with_dpapi(&cookie)?;
    config::save_credential(&encrypted).await?;

    // 关闭登录窗口
    window.close().map_err(|e| e.to_string())?;

    Ok("登录成功".to_string())
}

/// 监听登录状态
async fn wait_for_login(window: &WebviewWindow) -> Result<String, String> {
    // 注入 JS 监听 Cookie 变化
    window.eval(r#"
        // 定期检查是否已登录
        setInterval(() => {
            const cookies = document.cookie;
            if (cookies.includes('__puus') && cookies.includes('pus')) {
                window.__TAURI__.event.emit('login-success', { cookie: cookies });
            }
        }, 1000);
    "#)?;

    // 监听登录成功事件
    let cookie = app.listen("login-success", |event| {
        // 提取并返回 Cookie
    });

    Ok(cookie)
}
```

---

## 🧪 测试指南

### 测试结构

```
tests/
├── quark.test.ts            # 夸克 API 单元测试 (68 用例)
├── rpc.test.ts              # RPC 下载器测试 (42 用例)
├── config.test.ts           # 配置管理测试 (50 用例)
└── integration.test.ts      # 集成测试 (31 用例)
```

### 运行测试

```bash
# 运行所有前端测试
pnpm test

# 运行特定测试文件
pnpm test quark.test.ts

# 运行 Rust 后端测试
cd src-tauri && cargo test

# 运行带覆盖率报告的测试
pnpm test -- --coverage
```

### TDD 开发流程

本项目遵循 **TDD（测试驱动开发）** 流程：

```
1. RED    - 编写失败的测试
2. GREEN  - 编写最小代码使测试通过
3. REFACTOR - 重构代码，保持测试通过
```

### Mock 服务

开发时可使用 Mock 服务模拟夸克 API：

```typescript
// tests/mocks/quark.mock.ts
export function mockQuarkApi() {
  return {
    parseShareLink: () => mockShareInfo,
    transferFiles: () => mockTaskId,
    getDownloadLink: () => mockDownloadLink,
  };
}
```

---

## 📦 构建与发布

### 构建命令

```bash
# 开发构建（带调试信息）
pnpm tauri build --debug

# 生产构建
pnpm tauri build

# 仅构建 Windows 安装包
pnpm tauri build --bundles msi

# 仅构建 Windows 便携版
pnpm tauri build --bundles nsis
```

### 构建产物

| 格式 | 说明 | 位置 |
|------|------|------|
| `.msi` | Windows Installer 安装包 | `src-tauri/target/release/bundle/msi/` |
| `.exe` | NSIS 安装包 | `src-tauri/target/release/bundle/nsis/` |
| `.upx` | 便携版（免安装） | 可选配置 |

### 自动更新

使用 Tauri 内置的 updater 插件：

```json
// tauri.conf.json
{
  "plugins": {
    "updater": {
      "endpoints": [
        "https://your-update-server.com/update/{{target}}/{{arch}}/{{current_version}}"
      ],
      "pubkey": "YOUR_PUBLIC_KEY"
    }
  }
}
```

---

## 🤝 贡献指南

### 贡献流程

1. **Fork** 本仓库
2. 创建特性分支：`git checkout -b feature/your-feature`
3. **编写测试**（TDD：先写测试，再写实现）
4. 提交代码：`git commit -m 'feat: add your feature'`
5. 推送分支：`git push origin feature/your-feature`
6. 提交 **Pull Request**

### 提交规范

使用 [Conventional Commits](https://www.conventionalcommits.org/) 格式：

| 类型 | 说明 |
|------|------|
| `feat:` | 新功能 |
| `fix:` | 修复 Bug |
| `docs:` | 文档更新 |
| `style:` | 代码格式（不影响逻辑） |
| `refactor:` | 重构 |
| `test:` | 测试相关 |
| `chore:` | 构建/工具相关 |

### 代码规范

- **Rust**: 遵循 `cargo fmt` 和 `cargo clippy`
- **TypeScript**: 遵循 ESLint + Prettier 配置
- **组件**: 使用函数式组件 + Hooks
- **命名**: 英文命名，注释可用中文

---

<div align="center">

[← 返回主文档](README.md)

</div>
