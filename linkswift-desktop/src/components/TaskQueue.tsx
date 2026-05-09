/**
 * LinkSwift Desktop - 任务队列组件
 * 
 * 展示下载任务列表，显示任务状态和进度，支持重试和删除操作
 */

import React from 'react'
import type { DownloadTask } from '../types'

// 任务队列组件的属性接口
interface TaskQueueProps {
  tasks: DownloadTask[]               // 任务列表
  onRetry?: (id: string) => void       // 重试任务回调（可选）
  onRemove?: (id: string) => void       // 删除任务回调（可选）
}

/**
 * 根据任务状态获取状态显示文本
 * @param status - 任务状态对象
 * @returns 状态的中文描述文本
 */
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

/**
 * 根据任务状态获取状态图标
 * @param status - 任务状态对象
 * @returns 对应的表情符号图标
 */
function getStatusIcon(status: DownloadTask['status']): string {
  switch (status.type) {
    case 'Completed': return '✅'
    case 'Failed': return '❌'
    case 'Pending': return '⏳'
    default: return '🔄'
  }
}

/**
 * 任务队列组件
 * 展示下载任务列表，支持查看状态、重试和删除操作
 * @param tasks - 任务列表
 * @param onRetry - 重试任务回调
 * @param onRemove - 删除任务回调
 */
export function TaskQueue({ tasks, onRetry, onRemove }: TaskQueueProps) {
  return (
    <div className="task-queue" data-testid="task-queue">
      <h3>任务队列 ({tasks.length})</h3>
      {/* 任务列表 */}
      {tasks.map((task) => (
        <div key={task.id} className="task-item" data-testid={`task-item-${task.id}`}>
          {/* 状态图标 */}
          <span className="task-icon">{getStatusIcon(task.status)}</span>
          {/* 文件名或分享链接 */}
          <span className="task-name">{task.files.map(f => f.name).join(', ') || task.share_url}</span>
          {/* 状态文本 */}
          <span className="task-status" data-testid={`task-status-${task.id}`}>
            {getStatusText(task.status)}
          </span>
          {/* 转存进度条 */}
          {task.status.type === 'Transferring' && (
            <progress
              value={task.status.progress}
              max={100}
              data-testid={`task-progress-${task.id}`}
            />
          )}
          {/* 失败状态显示重试按钮 */}
          {task.status.type === 'Failed' && onRetry && (
            <button
              onClick={() => onRetry(task.id)}
              data-testid={`task-retry-${task.id}`}
            >
              重试
            </button>
          )}
          {/* 删除按钮 */}
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
