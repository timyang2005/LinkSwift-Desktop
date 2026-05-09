# LinkSwift Desktop - 软件设计规范

## 1. 概述

**项目名称**: LinkSwift Desktop
**项目类型**: Windows 桌面应用程序
**核心功能**: 网盘链接解析 → 文件选择 → 转存到夸克网盘 → 直链获取 → RPC下载
**目标用户**: 需要从分享链接快速下载文件到本地，并使用RPC下载器的用户

## 2. 功能列表

### 2.1 核心功能

| 功能模块 | 功能点 | 优先级 | 状态 |
|----------|--------|--------|------|
| **链接解析** | 解析夸克网盘分享链接 | P0 | 待设计 |
| **文件列表** | 获取分享文件列表，支持多级目录 | P0 | 待设计 |
| **文件选择** | 勾选文件/文件夹，支持全选 | P0 | 待设计 |
| **转存功能** | 将选中的文件转存到用户夸克网盘 | P0 | 待设计 |
| **直链获取** | 获取转存后文件的直链地址 | P0 | 待设计 |
| **RPC推送** | 推送直链到配置的RPC下载器 | P0 | 待设计 |
| **首次设置** | 首次运行时弹出RPC和Cookie配置界面 | P0 | 待设计 |

### 2.2 辅助功能

| 功能模块 | 功能点 | 优先级 | 状态 |
|----------|--------|--------|------|
| **Cookie管理** | 输入、验证、保存Cookie | P1 | 待设计 |
| **RPC配置** | 添加、编辑、删除RPC服务器配置 | P1 | 待设计 |
| **下载历史** | 显示任务执行历史和状态 | P2 | 待设计 |
| **界面主题** | 深色/浅色主题切换 | P2 | 待设计 |

## 3. 用户流程

```
┌─────────────────────────────────────────────────────────────────────┐
│                           用户流程图                                  │
└─────────────────────────────────────────────────────────────────────┘

[启动软件]
     │
     ▼
┌─────────────────┐
│ 首次运行检测    │
└────────┬────────┘
         │
    ┌────┴────┐
    │ 是      │ 否
    ▼         ▼
┌────────┐  ┌─────────────┐
│设置向导│  │ 主界面      │
└───┬────┘  └──────┬──────┘
    │              │
    ▼              ▼
┌──────────────┐  ┌──────────────┐
│ 1. Cookie输入 │  │ 1. 链接输入框 │
│ 2. RPC配置    │  │ 2. 解析按钮   │
│ 3. 保存退出   │  └──────┬───────┘
└──────────────┘         │
                         ▼
              ┌─────────────────────┐
              │ 解析分享链接         │
              └──────────┬──────────┘
                         │
                         ▼
              ┌─────────────────────┐
              │ 显示文件列表        │
              │ □ 文件1.txt        │
              │ ☑ 文件2.mp4        │
              │ ☑ 文件夹1/         │
              └──────────┬──────────┘
                         │
                         ▼
              ┌─────────────────────┐
              │ [转存] 按钮        │
              └──────────┬──────────┘
                         │
                         ▼
              ┌─────────────────────┐
              │ 转存到夸克网盘      │
              │ 进度显示...         │
              └──────────┬──────────┘
                         │
                         ▼
              ┌─────────────────────┐
              │ 获取直链            │
              └──────────┬──────────┘
                         │
                         ▼
              ┌─────────────────────┐
              │ 推送至RPC下载器     │
              │ ✅ 任务1 已推送     │
              │ ✅ 任务2 已推送     │
              └─────────────────────┘
```

## 4. 技术架构

### 4.1 技术选型

| 组件 | 选择 | 理由 |
|------|------|------|
| **框架** | Tauri 2.x | 轻量、安全、支持Windows原生 |
| **前端** | React + TypeScript | 类型安全、生态成熟 |
| **UI库** | shadcn/ui + TailwindCSS | 现代化组件库 |
| **状态管理** | Zustand | 轻量、简洁 |
| **HTTP客户端** | reqwest (Rust) | 高性能 |
| **存储** | SQLite (via rusqlite) | 本地数据持久化 |

### 4.2 项目结构

```
linkswift-desktop/
├── src/                      # React前端源码
│   ├── components/           # UI组件
│   │   ├── LinkInput.tsx    # 链接输入组件
│   │   ├── FileList.tsx     # 文件列表组件
│   │   ├── FileItem.tsx     # 文件项组件
│   │   ├── Settings.tsx     # 设置面板
│   │   └── FirstRunWizard.tsx # 首次运行向导
│   ├── hooks/               # React Hooks
│   │   ├── useQuark.ts      # 夸克API Hook
│   │   ├── useRPC.ts        # RPC下载 Hook
│   │   └── useConfig.ts     # 配置管理 Hook
│   ├── stores/              # Zustand状态
│   │   └── appStore.ts      # 应用状态
│   ├── lib/                 # 工具库
│   │   └── utils.ts         # 工具函数
│   ├── App.tsx              # 主应用
│   └── main.tsx             # 入口文件
├── src-tauri/               # Rust后端源码
│   ├── src/
│   │   ├── main.rs          # 入口
│   │   ├── lib.rs           # 库导出
│   │   ├── commands/        # Tauri命令
│   │   │   ├── mod.rs
│   │   │   ├── quark.rs     # 夸克网盘API
│   │   │   ├── rpc.rs        # RPC下载器
│   │   │   └── config.rs     # 配置管理
│   │   └── models/          # 数据模型
│   │       ├── mod.rs
│   │       ├── file.rs      # 文件模型
│   │       ├── share.rs     # 分享信息模型
│   │       └── config.rs     # 配置模型
│   ├── Cargo.toml           # Rust依赖
│   └── tauri.conf.json      # Tauri配置
├── tests/                   # 测试文件 (TDD)
│   ├── quark.test.ts        # 夸克API测试
│   ├── rpc.test.ts          # RPC测试
│   └── integration.test.ts   # 集成测试
├── package.json
├── tsconfig.json
├── vite.config.ts
└── SPEC.md                  # 本文档
```

## 5. 数据模型

### 5.1 配置模型

```rust
// src-tauri/src/models/config.rs

/// 应用配置
#[derive(Serialize, Deserialize, Clone)]
pub struct AppConfig {
    /// Cookie配置
    pub cookie: CookieConfig,
    /// RPC服务器列表
    pub rpc_servers: Vec<RpcServer>,
    /// 默认RPC服务器索引
    pub default_rpc_index: usize,
    /// 主题设置
    pub theme: Theme,
    /// 是否首次运行
    pub is_first_run: bool,
}

/// Cookie配置
#[derive(Serialize, Deserialize, Clone)]
pub struct CookieConfig {
    /// 夸克网盘Cookie
    pub quark_cookie: String,
    /// 最后验证时间
    pub last_verified: Option<i64>,
    /// 是否有效
    pub is_valid: bool,
}

/// RPC服务器
#[derive(Serialize, Deserialize, Clone)]
pub struct RpcServer {
    /// 服务器名称
    pub name: String,
    /// 服务器地址 (如 http://localhost:6800)
    pub url: String,
    /// RPC密钥
    pub token: Option<String>,
    /// 是否默认
    pub is_default: bool,
}

/// 主题设置
#[derive(Serialize, Deserialize, Clone)]
pub enum Theme {
    Light,
    Dark,
    System,
}
```

### 5.2 文件模型

```rust
// src-tauri/src/models/file.rs

/// 文件/文件夹项
#[derive(Serialize, Deserialize, Clone)]
pub struct FileItem {
    /// 文件ID
    pub fid: String,
    /// 文件名
    pub name: String,
    /// 是否文件夹
    pub is_folder: bool,
    /// 文件大小 (字节)
    pub size: u64,
    /// 父目录ID
    pub pdir_fid: String,
    /// MIME类型
    pub mime_type: Option<String>,
    /// 创建时间
    pub created_at: Option<i64>,
    /// 修改时间
    pub updated_at: Option<i64>,
    /// 是否选中
    #[serde(default)]
    pub selected: bool,
}

/// 分享信息
#[derive(Serialize, Deserialize, Clone)]
pub struct ShareInfo {
    /// 分享链接ID (pwd_id)
    pub pwd_id: String,
    /// 分享Token
    pub stoken: String,
    /// 分享标题
    pub title: String,
    /// 文件列表
    pub files: Vec<FileItem>,
}

/// 直链信息
#[derive(Serialize, Deserialize, Clone)]
pub struct DownloadLink {
    /// 文件ID
    pub fid: String,
    /// 文件名
    pub name: String,
    /// 直链URL
    pub url: String,
    /// 文件大小
    pub size: u64,
    /// MD5 (如果有)
    pub md5: Option<String>,
}
```

## 6. API设计 (Rust Commands)

### 6.1 夸克网盘API

```rust
// src-tauri/src/commands/quark.rs

/// 验证Cookie是否有效
#[tauri::command]
pub async fn verify_cookie(cookie: String) -> Result<bool, String>;

/// 获取分享信息
#[tauri::command]
pub async fn get_share_info(share_url: String, cookie: String) -> Result<ShareInfo, String>;

/// 获取分享文件列表
#[tauri::command]
pub async fn get_share_files(
    share_url: String,
    pwd_id: String,
    stoken: String,
    dir_id: String,
    cookie: String,
) -> Result<Vec<FileItem>, String>;

/// 转存文件
#[tauri::command]
pub async fn save_files(
    share_url: String,
    pwd_id: String,
    stoken: String,
    fid_list: Vec<String>,
    fid_token_list: Vec<String>,
    target_dir: String,
    cookie: String,
) -> Result<String, String>; // 返回任务ID

/// 查询转存任务状态
#[tauri::command]
pub async fn query_save_task(
    task_id: String,
    cookie: String,
) -> Result<SaveTaskStatus, String>;

/// 获取文件直链
#[tauri::command]
pub async fn get_download_url(
    file_id: String,
    cookie: String,
) -> Result<DownloadLink, String>;

/// 获取用户网盘文件列表 (用于选择转存目录)
#[tauri::command]
pub async fn get_user_files(
    dir_id: String,
    cookie: String,
) -> Result<Vec<FileItem>, String>;
```

### 6.2 RPC下载器API

```rust
// src-tauri/src/commands/rpc.rs

/// 添加RPC下载任务
#[tauri::command]
pub async fn add_rpc_task(
    rpc_url: String,
    token: Option<String>,
    urls: Vec<String>,
    filename: String,
    dir: Option<String>,
) -> Result<String, String>; // 返回任务ID

/// 测试RPC连接
#[tauri::command]
pub async fn test_rpc_connection(
    rpc_url: String,
    token: Option<String>,
) -> Result<bool, String>;
```

### 6.3 配置管理API

```rust
// src-tauri/src/commands/config.rs

/// 获取应用配置
#[tauri::command]
pub async fn get_config() -> Result<AppConfig, String>;

/// 保存应用配置
#[tauri::command]
pub async fn save_config(config: AppConfig) -> Result<(), String>;

/// 更新Cookie
#[tauri::command]
pub async fn update_cookie(cookie: String) -> Result<(), String>;

/// 添加RPC服务器
#[tauri::command]
pub async fn add_rpc_server(server: RpcServer) -> Result<(), String>;

/// 删除RPC服务器
#[tauri::command]
pub async fn delete_rpc_server(name: String) -> Result<(), String>;
```

## 7. Cookie获取教程

### 7.1 获取夸克网盘Cookie的步骤

1. **打开夸克网盘网页版**
   - 访问 https://pan.quark.cn
   - 登录你的账号

2. **打开开发者工具**
   - 按 `F12` 或 `Ctrl+Shift+I`
   - 切换到 **Network (网络)** 标签

3. **刷新页面并查找请求**
   - 按 `F5` 刷新页面
   - 在过滤器中输入 `pan.quark.cn`
   - 找一个任意 API 请求

4. **获取Cookie**
   - 点击该请求
   - 在 **Request Headers** 中找到 `cookie:` 字段
   - 复制整个 cookie 值（很长的一串字符串）

5. **粘贴到软件中**
   - 打开 LinkSwift Desktop
   - 进入设置
   - 粘贴到 Cookie 输入框
   - 点击"验证"按钮

### 7.2 重要提示

> ⚠️ **Cookie 有效期**
> - 夸克网盘Cookie通常有效期为 **30天左右**
> - 如果验证失败，需要重新获取新的Cookie
> - 建议定期检查Cookie有效性

> ⚠️ **Cookie安全**
> - Cookie相当于你的登录凭证
> - 不要将Cookie分享给他人
> - 仅在可信的软件中使用

## 8. 验收标准

### 8.1 首次运行

- [ ] 首次运行时自动弹出设置向导
- [ ] 设置向导包含Cookie输入和RPC配置
- [ ] 保存配置后自动关闭向导进入主界面
- [ ] 再次运行时不再显示向导

### 8.2 链接解析

- [ ] 输入夸克分享链接后能正确解析
- [ ] 支持带密码的分享链接（弹窗输入密码）
- [ ] 解析失败时显示友好的错误提示
- [ ] 加载过程显示进度指示

### 8.3 文件列表

- [ ] 正确显示文件/文件夹图标
- [ ] 显示文件名、大小、修改时间
- [ ] 支持勾选单个或多个文件
- [ ] 支持全选/取消全选
- [ ] 文件夹支持展开/折叠
- [ ] 支持多级目录导航

### 8.4 转存功能

- [ ] 转存前显示确认对话框
- [ ] 转存过程显示进度
- [ ] 转存完成后显示成功提示
- [ ] 转存失败时显示具体错误原因

### 8.5 RPC下载

- [ ] 能正确获取文件直链
- [ ] 成功推送任务到RPC服务器
- [ ] 显示推送结果（成功/失败）
- [ ] 支持配置多个RPC服务器

### 8.6 设置功能

- [ ] 可以修改Cookie
- [ ] 可以添加/编辑/删除RPC服务器
- [ ] 可以设置默认RPC服务器
- [ ] 配置变更后自动保存

## 9. 错误处理

### 9.1 错误类型

| 错误类型 | 错误码 | 处理方式 |
|----------|--------|----------|
| Cookie无效 | `E001` | 提示用户重新获取Cookie |
| 链接解析失败 | `E002` | 提示检查链接格式 |
| 分享已失效 | `E003` | 提示链接可能已过期 |
| 转存失败 | `E004` | 显示具体原因，提示重试 |
| RPC连接失败 | `E005` | 提示检查RPC配置 |
| 网络错误 | `E006` | 提示检查网络连接 |

### 9.2 用户提示示例

```
┌─────────────────────────────────────────┐
│  ❌ 操作失败                            │
├─────────────────────────────────────────┤
│  错误码: E001                           │
│  错误信息: Cookie已失效                 │
│                                         │
│  解决方法:                              │
│  请重新获取夸克网盘Cookie并更新设置     │
│                                         │
│  [查看获取教程]  [前往设置]  [关闭]     │
└─────────────────────────────────────────┘
```

## 10. 非功能需求

### 10.1 性能要求

- 应用启动时间: < 3秒
- 链接解析时间: < 5秒
- 文件列表加载: < 3秒
- 转存任务响应: < 1秒

### 10.2 兼容性要求

- Windows 10 (1903+)
- Windows 11
- 屏幕分辨率: 1280x720 起步

### 10.3 安全要求

- Cookie本地加密存储
- 不上传任何用户数据
- 网络请求使用HTTPS

## 11. 测试计划

### 11.1 单元测试 (Rust)

```rust
// tests/quark_test.rs

#[cfg(test)]
mod tests {
    use crate::commands::quark::*;

    #[test]
    fn test_parse_share_url() {
        let url = "https://pan.quark.cn/s/abc123";
        let result = parse_share_url(url);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().pwd_id, "abc123");
    }

    #[test]
    fn test_verify_cookie_format() {
        let cookie = "pdd_v=xxx; pc_pair_ticket=yyy; ...";
        assert!(validate_cookie(cookie));
    }
}
```

### 11.2 集成测试

```typescript
// tests/integration.test.ts

describe('完整流程测试', () => {
  it('应该完成 解析 -> 转存 -> 获取直链 -> RPC推送 流程', async () => {
    // 1. 验证Cookie
    const cookieValid = await verifyCookie(testCookie);
    expect(cookieValid).toBe(true);

    // 2. 解析链接
    const shareInfo = await getShareInfo(testShareUrl, testCookie);
    expect(shareInfo.files.length).toBeGreaterThan(0);

    // 3. 选择文件
    const selectedFiles = shareInfo.files.filter(f => f.selected);

    // 4. 转存
    const taskId = await saveFiles(selectedFiles, targetDir, testCookie);
    expect(taskId).toBeTruthy();

    // 5. 获取直链
    const downloadLink = await getDownloadUrl(taskId, testCookie);
    expect(downloadLink.url).toContain('http');

    // 6. 推送RPC
    const result = await addRpcTask(rpcConfig, downloadLink.url);
    expect(result.success).toBe(true);
  });
});
```

---

**文档版本**: 1.0.0
**创建日期**: 2026-05-09
**最后更新**: 2026-05-09
