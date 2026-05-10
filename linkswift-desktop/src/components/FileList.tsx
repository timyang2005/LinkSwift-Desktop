/**
 * LinkSwift Desktop - 文件列表组件
 * 
 * 展示文件列表，支持选择、全选、反选和展开文件夹操作
 */

import type { FileItem as FileItemType } from '../types'
import { formatFileSize } from '../lib/utils'

// 单个文件项组件的属性接口
interface FileItemProps {
  item: FileItemType                  // 文件数据
  onToggle: (fid: string) => void      // 切换选中状态的回调
  onExpand?: (fid: string) => void     // 展开文件夹的回调（可选）
}

/**
 * 单个文件项组件
 * 显示文件的复选框、图标、名称和大小
 * @param item - 文件数据
 * @param onToggle - 切换选中状态回调
 * @param onExpand - 展开文件夹回调
 */
export function FileItemComponent({ item, onToggle, onExpand }: FileItemProps) {
  return (
    <div className="file-item" data-testid={`file-item-${item.fid}`}>
      {/* 文件复选框 */}
      <input
        type="checkbox"
        checked={item.selected}
        onChange={() => onToggle(item.fid)}
        data-testid={`file-checkbox-${item.fid}`}
      />
      {/* 文件图标，点击可展开文件夹 */}
      <span
        className="file-icon"
        data-testid={`file-icon-${item.fid}`}
        onClick={item.is_folder && onExpand ? () => onExpand(item.fid) : undefined}
      >
        {item.is_folder ? '📁' : '📄'}
      </span>
      {/* 文件名称 */}
      <span className="file-name" data-testid={`file-name-${item.fid}`}>
        {item.name}
      </span>
      {/* 文件大小 */}
      <span className="file-size" data-testid={`file-size-${item.fid}`}>
        {formatFileSize(item.size)}
      </span>
    </div>
  )
}

// 文件列表组件的属性接口
interface FileListProps {
  files: FileItemType[]               // 文件列表数据
  onToggle: (fid: string) => void      // 切换文件选中状态回调
  onSelectAll: () => void             // 全选所有文件回调
  onDeselectAll: () => void           // 取消所有选中回调
  onExpand?: (fid: string) => void    // 展开文件夹回调（可选）
}

/**
 * 文件列表组件
 * 包含全选/反选按钮和文件列表
 * @param files - 文件列表数据
 * @param onToggle - 切换文件选中状态回调
 * @param onSelectAll - 全选所有文件回调
 * @param onDeselectAll - 取消所有选中回调
 * @param onExpand - 展开文件夹回调
 */
export function FileList({ files, onToggle, onSelectAll, onDeselectAll, onExpand }: FileListProps) {
  return (
    <div className="file-list" data-testid="file-list">
      {/* 操作按钮区域 */}
      <div className="file-list-actions">
        <button onClick={onSelectAll} data-testid="select-all-button">全选</button>
        <button onClick={onDeselectAll} data-testid="deselect-all-button">反选</button>
      </div>
      {/* 文件列表 */}
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
