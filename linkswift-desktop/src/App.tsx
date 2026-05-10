import { useState } from 'react'
import type { ShareInfo, FileItem } from './types'
import { useAppStore } from './stores/appStore'
import { LinkInput } from './components/LinkInput'
import { FileList } from './components/FileList'
import { TaskQueue } from './components/TaskQueue'
import { Settings } from './components/Settings'
import { FirstRunWizard } from './components/FirstRunWizard'
import './App.css'

function App() {
  const {
    config,
    shareInfo,
    taskQueue,
    isLoading,
    setShareInfo,
    toggleFileSelection,
    selectAllFiles,
    deselectAllFiles,
    removeTask,
    updateTaskStatus,
    setLoading,
    setError,
    setFirstRunComplete,
    addRpcServer,
    removeRpcServer,
  } = useAppStore()

  const [showSettings, setShowSettings] = useState(false)

  const handleParse = async (url: string) => {
    setLoading(true)
    setError(null)
    try {
      const { invoke } = await import('@tauri-apps/api/core')
      const result = await invoke<ShareInfo>('parse_share_link', { shareUrl: url })
      setShareInfo(result)
    } catch (e) {
      setError(String(e))
    } finally {
      setLoading(false)
    }
  }

  const handleLogin = async (): Promise<boolean> => {
    try {
      const { invoke } = await import('@tauri-apps/api/core')
      await invoke('open_login_window')
      return true
    } catch {
      return false
    }
  }

  const handleTestConnection = async (url: string, token?: string): Promise<boolean> => {
    try {
      const { invoke } = await import('@tauri-apps/api/core')
      await invoke('test_rpc_connection', { url, token: token || null })
      return true
    } catch {
      return false
    }
  }

  const handleRetry = async (id: string) => {
    updateTaskStatus(id, { type: 'Pending' })
  }

  const handleExpand = async (fid: string) => {
    if (!shareInfo) return
    setLoading(true)
    try {
      const { invoke } = await import('@tauri-apps/api/core')
      const result = await invoke<{ items: FileItem[] }>('get_share_files', {
        pwdId: shareInfo.pwd_id,
        stoken: shareInfo.stoken,
        dirFid: fid,
        page: 1,
        size: 100,
      })
      const newFiles = result.items || []
      setShareInfo({
        ...shareInfo,
        files: [...shareInfo.files, ...newFiles],
      })
    } catch (e) {
      setError(String(e))
    } finally {
      setLoading(false)
    }
  }

  if (config.is_first_run) {
    return (
      <div className="app">
        <FirstRunWizard
          onComplete={setFirstRunComplete}
          onLogin={handleLogin}
          onTestConnection={handleTestConnection}
        />
      </div>
    )
  }

  return (
    <div className="app">
      <header className="app-header">
        <h1>LinkSwift Desktop</h1>
        <button
          className="settings-toggle"
          onClick={() => setShowSettings(!showSettings)}
          data-testid="settings-toggle"
        >
          {showSettings ? '返回' : '设置'}
        </button>
      </header>

      {showSettings ? (
        <Settings
          config={config}
          onAddServer={addRpcServer}
          onDeleteServer={removeRpcServer}
          onTestConnection={handleTestConnection}
          onReLogin={handleLogin}
        />
      ) : (
        <main className="app-main">
          <LinkInput onParse={handleParse} isLoading={isLoading} />

          {shareInfo && (
            <FileList
              files={shareInfo.files}
              onToggle={toggleFileSelection}
              onSelectAll={selectAllFiles}
              onDeselectAll={deselectAllFiles}
              onExpand={handleExpand}
            />
          )}

          {taskQueue.length > 0 && (
            <TaskQueue
              tasks={taskQueue}
              onRetry={handleRetry}
              onRemove={removeTask}
            />
          )}
        </main>
      )}
    </div>
  )
}

export default App
