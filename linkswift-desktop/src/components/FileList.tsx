import React from 'react'
import type { FileItem as FileItemType } from '../types'
import { formatFileSize } from '../lib/utils'

interface FileItemProps {
  item: FileItemType
  onToggle: (fid: string) => void
  onExpand?: (fid: string) => void
}

export function FileItemComponent({ item, onToggle, onExpand }: FileItemProps) {
  return (
    <div className="file-item" data-testid={`file-item-${item.fid}`}>
      <input
        type="checkbox"
        checked={item.selected}
        onChange={() => onToggle(item.fid)}
        data-testid={`file-checkbox-${item.fid}`}
      />
      <span
        className="file-icon"
        data-testid={`file-icon-${item.fid}`}
        onClick={item.is_folder && onExpand ? () => onExpand(item.fid) : undefined}
      >
        {item.is_folder ? '📁' : '📄'}
      </span>
      <span className="file-name" data-testid={`file-name-${item.fid}`}>
        {item.name}
      </span>
      <span className="file-size" data-testid={`file-size-${item.fid}`}>
        {formatFileSize(item.size)}
      </span>
    </div>
  )
}

interface FileListProps {
  files: FileItemType[]
  onToggle: (fid: string) => void
  onSelectAll: () => void
  onDeselectAll: () => void
  onExpand?: (fid: string) => void
}

export function FileList({ files, onToggle, onSelectAll, onDeselectAll, onExpand }: FileListProps) {
  return (
    <div className="file-list" data-testid="file-list">
      <div className="file-list-actions">
        <button onClick={onSelectAll} data-testid="select-all-button">全选</button>
        <button onClick={onDeselectAll} data-testid="deselect-all-button">反选</button>
      </div>
      {files.map((file) => (
        <FileItemComponent
          key={file.fid}
          item={file}
          onToggle={onToggle}
          onExpand={onExpand}
        />
      ))}
    </div>
  )
}
