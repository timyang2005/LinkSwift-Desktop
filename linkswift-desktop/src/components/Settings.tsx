/**
 * LinkSwift Desktop - 设置组件
 * 
 * 提供应用设置界面，包括账号管理和 RPC 下载器配置
 */

import { useState } from 'react'
import type { RpcServer } from '../types'

// 设置组件的属性接口
interface SettingsProps {
  config: {
    credential: { is_valid: boolean; last_verified: number }  // 登录凭证状态
    rpc_servers: RpcServer[]                                    // RPC 服务器列表
  }
  onAddServer: (server: RpcServer) => void                      // 添加服务器回调
  onDeleteServer: (id: string) => void                         // 删除服务器回调
  onTestConnection: (url: string, token?: string) => Promise<boolean>  // 测试连接回调
  onReLogin: () => void                                        // 重新登录回调
}

/**
 * 设置组件
 * 提供账号管理和 RPC 服务器配置功能
 * @param config - 当前配置
 * @param onAddServer - 添加服务器回调
 * @param onDeleteServer - 删除服务器回调
 * @param onTestConnection - 测试连接回调
 * @param onReLogin - 重新登录回调
 */
export function Settings({ config, onAddServer, onDeleteServer, onTestConnection, onReLogin }: SettingsProps) {
  // 新服务器名称
  const [newServerName, setNewServerName] = useState('')
  // 新服务器地址
  const [newServerUrl, setNewServerUrl] = useState('')
  // 新服务器密钥
  const [newServerToken, setNewServerToken] = useState('')
  // 测试结果
  const [testResult, setTestResult] = useState<boolean | null>(null)
  // 是否正在测试
  const [isTesting, setIsTesting] = useState(false)

  // 添加新的 RPC 服务器
  const handleAddServer = () => {
    if (!newServerName.trim() || !newServerUrl.trim()) return
    onAddServer({
      id: crypto.randomUUID(),
      name: newServerName,
      url: newServerUrl,
      token: newServerToken || undefined,
      downloader_type: 'Aria2',
      is_default: config.rpc_servers.length === 0,
    })
    setNewServerName('')
    setNewServerUrl('')
    setNewServerToken('')
  }

  // 测试连接
  const handleTestConnection = async () => {
    if (!newServerUrl.trim()) return
    setIsTesting(true)
    try {
      const result = await onTestConnection(newServerUrl, newServerToken || undefined)
      setTestResult(result)
    } finally {
      setIsTesting(false)
    }
  }

  return (
    <div className="settings" data-testid="settings-panel">
      <h2>设置</h2>

      {/* 账号管理区域 */}
      <section data-testid="credential-section">
        <h3>账号管理</h3>
        <div data-testid="cookie-status">
          登录状态: {config.credential.is_valid ? '✅ 已登录' : '❌ 未登录'}
        </div>
        <button onClick={onReLogin} data-testid="re-login-button">重新登录</button>
      </section>

      {/* RPC 下载器配置区域 */}
      <section data-testid="rpc-section">
        <h3>RPC 下载器</h3>
        {/* 服务器列表 */}
        {config.rpc_servers.map((server) => (
          <div key={server.id} data-testid={`rpc-server-${server.id}`}>
            <span>{server.name} - {server.url}</span>
            <button onClick={() => onDeleteServer(server.id)} data-testid={`delete-server-${server.id}`}>
              删除
            </button>
          </div>
        ))}

        {/* 添加服务器表单 */}
        <div data-testid="add-server-form">
          <input
            value={newServerName}
            onChange={(e) => setNewServerName(e.target.value)}
            placeholder="服务器名称"
            data-testid="new-server-name"
          />
          <input
            value={newServerUrl}
            onChange={(e) => setNewServerUrl(e.target.value)}
            placeholder="RPC 地址"
            data-testid="new-server-url"
          />
          <input
            value={newServerToken}
            onChange={(e) => setNewServerToken(e.target.value)}
            placeholder="RPC 密钥 (可选)"
            data-testid="new-server-token"
          />
          <button onClick={handleTestConnection} disabled={isTesting} data-testid="test-connection-button">
            {isTesting ? '测试中...' : '测试连接'}
          </button>
          {/* 测试结果提示 */}
          {testResult !== null && (
            <div data-testid="test-connection-result">
              {testResult ? '✅ 连接成功' : '❌ 连接失败'}
            </div>
          )}
          <button onClick={handleAddServer} data-testid="add-server-button">添加</button>
        </div>
      </section>
    </div>
  )
}
