import React from 'react'
import type { DownloadTask } from '../types'

interface TaskQueueProps {
  tasks: DownloadTask[]
  onRetry?: (id: string) => void
  onRemove?: (id: string) => void
}

function getStatusText(status: DownloadTask['status']): string {
  switch (status.type) {
    case 'Pending': return '等待中'
    case 'Parsing': return '解析中'
    case 'Transferring': return `转存中 ${status.progress}%`
    case 'FetchingLink': return '获取直链'
    case 'Pushing': return '推送中'
    case 'Completed': return '已完成'
    case 'Failed': return `失败: ${status.reason}`
    case 'Cancelled': return '已取消'
  }
}

function getStatusIcon(status: DownloadTask['status']): string {
  switch (status.type) {
    case 'Completed': return '✅'
    case 'Failed': return '❌'
    case 'Pending': return '⏳'
    default: return '🔄'
  }
}

export function TaskQueue({ tasks, onRetry, onRemove }: TaskQueueProps) {
  return (
    <div className="task-queue" data-testid="task-queue">
      <h3>任务队列 ({tasks.length})</h3>
      {tasks.map((task) => (
        <div key={task.id} className="task-item" data-testid={`task-item-${task.id}`}>
          <span className="task-icon">{getStatusIcon(task.status)}</span>
          <span className="task-name">{task.files.map(f => f.name).join(', ') || task.share_url}</span>
          <span className="task-status" data-testid={`task-status-${task.id}`}>
            {getStatusText(task.status)}
          </span>
          {task.status.type === 'Transferring' && (
            <progress
              value={task.status.progress}
              max={100}
              data-testid={`task-progress-${task.id}`}
            />
          )}
          {task.status.type === 'Failed' && onRetry && (
            <button
              onClick={() => onRetry(task.id)}
              data-testid={`task-retry-${task.id}`}
            >
              重试
            </button>
          )}
          {onRemove && (
            <button
              onClick={() => onRemove(task.id)}
              data-testid={`task-remove-${task.id}`}
            >
              删除
            </button>
          )}
        </div>
      ))}
    </div>
  )
}
