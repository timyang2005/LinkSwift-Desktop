# LinkSwift Desktop

网盘分享链接 → 一键转存 → 自动推送下载

## 功能特性

- **夸克网盘分享链接解析**: 自动解析夸克网盘分享链接
- **RPC 下载支持**: 支持 Aria2 等 RPC 下载工具
- **任务队列管理**: 支持多个下载任务并行管理
- **跨平台支持**: Windows、macOS、Linux

## 技术栈

- **前端**: React 19 + TypeScript + Zustand
- **后端**: Rust (Tauri 2.x)
- **构建工具**: Vite + pnpm

## 开发

```bash
# 安装依赖
pnpm install

# 开发模式
pnpm dev

# 构建
pnpm build

# 运行测试
pnpm test

# Rust 测试
pnpm test:rust
```

## CI/CD

本项目使用 GitHub Actions 进行持续集成和部署：
- CI: 每次 PR 和 push 到 main 分支自动运行测试
- Release: 推送 v* 标签自动构建并发布

## License

AGPL-3.0
