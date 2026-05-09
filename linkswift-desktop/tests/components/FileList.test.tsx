import { describe, test, expect, vi } from 'vitest'
import { render, screen, fireEvent } from '@testing-library/react'
import { FileList } from '../../src/components/FileList'
import type { FileItem } from '../../src/types'

const mockFiles: FileItem[] = [
  {
    fid: 'f1',
    name: 'movie.mp4',
    is_folder: false,
    size: 1073741824,
    pdir_fid: '0',
    selected: false,
  },
  {
    fid: 'f2',
    name: 'doc.txt',
    is_folder: false,
    size: 256,
    pdir_fid: '0',
    selected: true,
  },
  {
    fid: 'dir1',
    name: '文件夹',
    is_folder: true,
    size: 0,
    pdir_fid: '0',
    selected: false,
  },
]

describe('FileList 组件', () => {
  test('渲染文件列表', () => {
    render(
      <FileList
        files={mockFiles}
        onToggle={vi.fn()}
        onSelectAll={vi.fn()}
        onDeselectAll={vi.fn()}
      />
    )
    expect(screen.getByTestId('file-list')).toBeInTheDocument()
    expect(screen.getByText('movie.mp4')).toBeInTheDocument()
    expect(screen.getByText('doc.txt')).toBeInTheDocument()
    expect(screen.getByText('文件夹')).toBeInTheDocument()
  })

  test('文件显示正确的大小格式', () => {
    render(
      <FileList
        files={mockFiles}
        onToggle={vi.fn()}
        onSelectAll={vi.fn()}
        onDeselectAll={vi.fn()}
      />
    )
    expect(screen.getByTestId('file-size-f1')).toHaveTextContent('1.0 GB')
    expect(screen.getByTestId('file-size-f2')).toHaveTextContent('256 B')
  })

  test('文件夹显示文件夹图标', () => {
    render(
      <FileList
        files={mockFiles}
        onToggle={vi.fn()}
        onSelectAll={vi.fn()}
        onDeselectAll={vi.fn()}
      />
    )
    expect(screen.getByTestId('file-icon-dir1')).toHaveTextContent('📁')
  })

  test('文件显示文件图标', () => {
    render(
      <FileList
        files={mockFiles}
        onToggle={vi.fn()}
        onSelectAll={vi.fn()}
        onDeselectAll={vi.fn()}
      />
    )
    expect(screen.getByTestId('file-icon-f1')).toHaveTextContent('📄')
  })

  test('点击复选框触发 onToggle', () => {
    const onToggle = vi.fn()
    render(
      <FileList
        files={mockFiles}
        onToggle={onToggle}
        onSelectAll={vi.fn()}
        onDeselectAll={vi.fn()}
      />
    )
    fireEvent.click(screen.getByTestId('file-checkbox-f1'))
    expect(onToggle).toHaveBeenCalledWith('f1')
  })

  test('全选按钮触发 onSelectAll', () => {
    const onSelectAll = vi.fn()
    render(
      <FileList
        files={mockFiles}
        onToggle={vi.fn()}
        onSelectAll={onSelectAll}
        onDeselectAll={vi.fn()}
      />
    )
    fireEvent.click(screen.getByTestId('select-all-button'))
    expect(onSelectAll).toHaveBeenCalled()
  })

  test('反选按钮触发 onDeselectAll', () => {
    const onDeselectAll = vi.fn()
    render(
      <FileList
        files={mockFiles}
        onToggle={vi.fn()}
        onSelectAll={vi.fn()}
        onDeselectAll={onDeselectAll}
      />
    )
    fireEvent.click(screen.getByTestId('deselect-all-button'))
    expect(onDeselectAll).toHaveBeenCalled()
  })

  test('已选中的文件 checkbox 为 checked', () => {
    render(
      <FileList
        files={mockFiles}
        onToggle={vi.fn()}
        onSelectAll={vi.fn()}
        onDeselectAll={vi.fn()}
      />
    )
    expect(screen.getByTestId('file-checkbox-f2')).toBeChecked()
    expect(screen.getByTestId('file-checkbox-f1')).not.toBeChecked()
  })

  test('点击文件夹触发 onExpand', () => {
    const onExpand = vi.fn()
    render(
      <FileList
        files={mockFiles}
        onToggle={vi.fn()}
        onSelectAll={vi.fn()}
        onDeselectAll={vi.fn()}
        onExpand={onExpand}
      />
    )
    fireEvent.click(screen.getByTestId('file-icon-dir1'))
    expect(onExpand).toHaveBeenCalledWith('dir1')
  })

  test('空文件列表渲染', () => {
    render(
      <FileList
        files={[]}
        onToggle={vi.fn()}
        onSelectAll={vi.fn()}
        onDeselectAll={vi.fn()}
      />
    )
    expect(screen.getByTestId('file-list')).toBeInTheDocument()
  })
})
