/**
 * LinkSwift Desktop - 首次运行向导组件
 * 
 * 引导用户完成初始配置，包括登录夸克网盘和配置 RPC 下载器
 */

import { useState } from 'react'

// 向导组件的属性接口
interface FirstRunWizardProps {
  onComplete: () => void                                // 完成回调
  onLogin: () => Promise<boolean>                      // 登录回调
  onTestConnection: (url: string, token?: string) => Promise<boolean>  // 测试连接回调
}

/**
 * 首次运行向导组件
 * 包含 3 个步骤：登录、配置 RPC、完成
 * @param onComplete - 完成后回调
 * @param onLogin - 登录回调
 * @param onTestConnection - 测试连接回调
 */
export function FirstRunWizard({ onComplete, onLogin, onTestConnection }: FirstRunWizardProps) {
  // 当前步骤（1-3）
  const [step, setStep] = useState(1)
  // 是否已登录
  const [isLoggedIn, setIsLoggedIn] = useState(false)
  // RPC 服务器地址
  const [rpcUrl, setRpcUrl] = useState('http://localhost:6800')
  // RPC 密钥
  const [rpcToken, setRpcToken] = useState('')
  // 连接是否已测试
  const [connectionTested, setConnectionTested] = useState(false)

  // 处理登录操作
  const handleLogin = async () => {
    const result = await onLogin()
    setIsLoggedIn(result)
  }

  // 处理测试连接操作
  const handleTestConnection = async () => {
    const result = await onTestConnection(rpcUrl, rpcToken || undefined)
    setConnectionTested(result)
  }

  // 判断是否可以进入下一步
  const canGoNext = step === 1 ? isLoggedIn : connectionTested

  return (
    <div className="wizard" data-testid="first-run-wizard">
      <h2>欢迎使用 LinkSwift Desktop ({step}/3)</h2>

      {/* 步骤 1：登录夸克网盘 */}
      {step === 1 && (
        <div data-testid="wizard-step-login">
          <h3>登录夸克网盘</h3>
          <button onClick={handleLogin} data-testid="wizard-login-button">
            {isLoggedIn ? '✅ 已登录' : '扫码登录'}
          </button>
        </div>
      )}

      {/* 步骤 2：配置 RPC 下载器 */}
      {step === 2 && (
        <div data-testid="wizard-step-rpc">
          <h3>配置 RPC 下载器</h3>
          <input
            value={rpcUrl}
            onChange={(e) => setRpcUrl(e.target.value)}
            placeholder="RPC 地址"
            data-testid="wizard-rpc-url"
          />
          <input
            value={rpcToken}
            onChange={(e) => setRpcToken(e.target.value)}
            placeholder="RPC 密钥 (可选)"
            data-testid="wizard-rpc-token"
          />
          <button onClick={handleTestConnection} data-testid="wizard-test-connection">
            测试连接
          </button>
          {connectionTested && <div data-testid="wizard-connection-ok">✅ 连接成功</div>}
        </div>
      )}

      {/* 步骤 3：配置完成 */}
      {step === 3 && (
        <div data-testid="wizard-step-done">
          <h3>配置完成</h3>
          <p>一切就绪，开始使用吧！</p>
        </div>
      )}

      {/* 向导操作按钮 */}
      <div className="wizard-actions">
        {step > 1 && (
          <button onClick={() => setStep(step - 1)} data-testid="wizard-prev">
            上一步
          </button>
        )}
        {step < 3 ? (
          <button
            onClick={() => setStep(step + 1)}
            disabled={!canGoNext}
            data-testid="wizard-next"
          >
            下一步
          </button>
        ) : (
          <button onClick={onComplete} data-testid="wizard-complete">
            完成
          </button>
        )}
      </div>
    </div>
  )
}
