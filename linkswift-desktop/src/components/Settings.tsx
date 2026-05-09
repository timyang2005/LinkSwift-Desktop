import React, { useState } from 'react'
import type { RpcServer } from '../types'

interface SettingsProps {
  config: {
    credential: { is_valid: boolean; last_verified: number }
    rpc_servers: RpcServer[]
  }
  onAddServer: (server: RpcServer) => void
  onDeleteServer: (id: string) => void
  onTestConnection: (url: string, token?: string) => Promise<boolean>
  onReLogin: () => void
}

export function Settings({ config, onAddServer, onDeleteServer, onTestConnection, onReLogin }: SettingsProps) {
  const [newServerName, setNewServerName] = useState('')
  const [newServerUrl, setNewServerUrl] = useState('')
  const [newServerToken, setNewServerToken] = useState('')
  const [testResult, setTestResult] = useState<boolean | null>(null)
  const [isTesting, setIsTesting] = useState(false)

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

      <section data-testid="credential-section">
        <h3>账号管理</h3>
        <div data-testid="cookie-status">
          登录状态: {config.credential.is_valid ? '✅ 已登录' : '❌ 未登录'}
        </div>
        <button onClick={onReLogin} data-testid="re-login-button">重新登录</button>
      </section>

      <section data-testid="rpc-section">
        <h3>RPC 下载器</h3>
        {config.rpc_servers.map((server) => (
          <div key={server.id} data-testid={`rpc-server-${server.id}`}>
            <span>{server.name} - {server.url}</span>
            <button onClick={() => onDeleteServer(server.id)} data-testid={`delete-server-${server.id}`}>
              删除
            </button>
          </div>
        ))}

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
