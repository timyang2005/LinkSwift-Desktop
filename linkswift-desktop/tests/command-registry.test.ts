import { describe, test, expect } from 'vitest'

const BACKEND_COMMANDS = [
  'parse_share_link',
  'submit_share_password',
  'get_share_files',
  'transfer_files',
  'query_transfer_task',
  'get_download_link',
  'verify_credential',
  'get_user_directories',
  'add_download_task',
  'test_rpc_connection',
  'query_rpc_task_status',
  'get_config',
  'save_config',
  'add_rpc_server',
  'update_rpc_server',
  'delete_rpc_server',
  'set_default_rpc_server',
  'open_login_window',
  'verify_credential_status',
]

const FRONTEND_INVOKE_COMMANDS = [
  'parse_share_link',
  'open_login_window',
  'test_rpc_connection',
  'get_share_files',
]

describe('Tauri 命令注册一致性', () => {
  test('前端调用的所有命令在后端都有定义', () => {
    for (const cmd of FRONTEND_INVOKE_COMMANDS) {
      expect(
        BACKEND_COMMANDS.includes(cmd),
        `前端调用命令 "${cmd}" 在后端未找到定义`
      ).toBe(true)
    }
  })

  test('后端命令 parse_share_link 应被前端使用', () => {
    expect(FRONTEND_INVOKE_COMMANDS).toContain('parse_share_link')
  })

  test('后端命令 open_login_window 应被前端使用', () => {
    expect(FRONTEND_INVOKE_COMMANDS).toContain('open_login_window')
  })

  test('前端不应使用后端不存在的命令名', () => {
    const invalidCommands = FRONTEND_INVOKE_COMMANDS.filter(
      cmd => !BACKEND_COMMANDS.includes(cmd)
    )
    expect(
      invalidCommands,
      `以下前端命令在后端不存在: ${invalidCommands.join(', ')}`
    ).toHaveLength(0)
  })
})
