/**
 * LinkSwift Desktop - React 应用入口文件
 * 
 * 该文件负责初始化 React 应用并将其挂载到 DOM 中
 */

import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './index.css'
import App from './App.tsx'

// 创建 React 根节点并将 App 组件渲染到 DOM 中
// StrictMode 会在开发环境中启用额外的检查和警告
createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <App />
  </StrictMode>,
)
