import React, { useState } from 'react'

interface FirstRunWizardProps {
  onComplete: () => void
  onLogin: () => Promise<boolean>
  onTestConnection: (url: string, token?: string) => Promise<boolean>
}

export function FirstRunWizard({ onComplete, onLogin, onTestConnection }: FirstRunWizardProps) {
  const [step, setStep] = useState(1)
  const [isLoggedIn, setIsLoggedIn] = useState(false)
  const [rpcUrl, setRpcUrl] = useState('http://localhost:6800')
  const [rpcToken, setRpcToken] = useState('')
  const [connectionTested, setConnectionTested] = useState(false)

  const handleLogin = async () => {
    const result = await onLogin()
    setIsLoggedIn(result)
  }

  const handleTestConnection = async () => {
    const result = await onTestConnection(rpcUrl, rpcToken || undefined)
    setConnectionTested(result)
  }

  const canGoNext = step === 1 ? isLoggedIn : connectionTested

  return (
    <div className="wizard" data-testid="first-run-wizard">
      <h2>欢迎使用 LinkSwift Desktop ({step}/3)</h2>

      {step === 1 && (
        <div data-testid="wizard-step-login">
          <h3>登录夸克网盘</h3>
          <button onClick={handleLogin} data-testid="wizard-login-button">
            {isLoggedIn ? '✅ 已登录' : '扫码登录'}
          </button>
        </div>
      )}

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

      {step === 3 && (
        <div data-testid="wizard-step-done">
          <h3>配置完成</h3>
          <p>一切就绪，开始使用吧！</p>
        </div>
      )}

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
