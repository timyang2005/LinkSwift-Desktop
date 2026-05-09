import { describe, test, expect, vi } from 'vitest'
import { render, screen, fireEvent } from '@testing-library/react'
import { TaskQueue } from '../../src/components/TaskQueue'
import type { DownloadTask } from '../../src/types'

const mockTasks: DownloadTask[] = [
  {
    id: 'task-001',
    share_url: 'https://pan.quark.cn/s/abc',
    files: [{ fid: 'f1', name: 'movie.mp4', is_folder: false, size: 1024, pdir_fid: '0', selected: true }],
    status: { type: 'Completed' },
    target_dir: '0',
    rpc_server_id: 'srv1',
    created_at: 1700000000,
    retry_count: 0,
  },
  {
    id: 'task-002',
    share_url: 'https://pan.quark.cn/s/xyz',
    files: [{ fid: 'f2', name: 'doc.txt', is_folder: false, size: 256, pdir_fid: '0', selected: true }],
    status: { type: 'Transferring', progress: 67 },
    target_dir: '0',
    rpc_server_id: 'srv1',
    created_at: 1700000000,
    retry_count: 0,
  },
  {
    id: 'task-003',
    share_url: 'https://pan.quark.cn/s/err',
    files: [],
    status: { type: 'Failed', reason: 'Cookie已失效' },
    target_dir: '0',
    rpc_server_id: 'srv1',
    created_at: 1700000000,
    retry_count: 3,
  },
]

describe('TaskQueue 组件', () => {
  test('渲染任务队列', () => {
    render(<TaskQueue tasks={mockTasks} />)
    expect(screen.getByTestId('task-queue')).toBeInTheDocument()
  })

  test('显示任务数量', () => {
    render(<TaskQueue tasks={mockTasks} />)
    expect(screen.getByText(/任务队列 \(3\)/)).toBeInTheDocument()
  })

  test('已完成任务显示完成图标', () => {
    render(<TaskQueue tasks={mockTasks} />)
    expect(screen.getByTestId('task-status-task-001')).toHaveTextContent('已完成')
  })

  test('进行中任务显示进度', () => {
    render(<TaskQueue tasks={mockTasks} />)
    expect(screen.getByTestId('task-status-task-002')).toHaveTextContent('转存中 67%')
  })

  test('进行中任务显示进度条', () => {
    render(<TaskQueue tasks={mockTasks} />)
    expect(screen.getByTestId('task-progress-task-002')).toBeInTheDocument()
  })

  test('失败任务显示错误原因', () => {
    render(<TaskQueue tasks={mockTasks} />)
    expect(screen.getByTestId('task-status-task-003')).toHaveTextContent('Cookie已失效')
  })

  test('失败任务显示重试按钮', () => {
    const onRetry = vi.fn()
    render(<TaskQueue tasks={mockTasks} onRetry={onRetry} />)
    expect(screen.getByTestId('task-retry-task-003')).toBeInTheDocument()
    fireEvent.click(screen.getByTestId('task-retry-task-003'))
    expect(onRetry).toHaveBeenCalledWith('task-003')
  })

  test('已完成任务不显示重试按钮', () => {
    render(<TaskQueue tasks={mockTasks} onRetry={vi.fn()} />)
    expect(screen.queryByTestId('task-retry-task-001')).not.toBeInTheDocument()
  })

  test('删除按钮触发 onRemove', () => {
    const onRemove = vi.fn()
    render(<TaskQueue tasks={mockTasks} onRemove={onRemove} />)
    fireEvent.click(screen.getByTestId('task-remove-task-001'))
    expect(onRemove).toHaveBeenCalledWith('task-001')
  })

  test('空任务队列', () => {
    render(<TaskQueue tasks={[]} />)
    expect(screen.getByText(/任务队列 \(0\)/)).toBeInTheDocument()
  })
})
