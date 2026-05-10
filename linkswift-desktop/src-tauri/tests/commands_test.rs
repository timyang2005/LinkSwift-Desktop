use pretty_assertions::assert_eq;

#[tokio::test]
async fn command_parse_share_link_valid_url() {
    let result =
        app_lib::commands::quark::parse_share_link("https://pan.quark.cn/s/abc123".to_string())
            .await;
    assert!(result.is_ok(), "valid URL should parse successfully");
    let info = result.unwrap();
    assert_eq!(info.pwd_id, "abc123");
}

#[tokio::test]
async fn command_parse_share_link_invalid_url() {
    let result =
        app_lib::commands::quark::parse_share_link("https://example.com/not-quark".to_string())
            .await;
    assert!(result.is_err(), "invalid URL should return error");
}

#[tokio::test]
async fn command_parse_share_link_empty_url() {
    let result = app_lib::commands::quark::parse_share_link("".to_string()).await;
    assert!(result.is_err());
}

#[tokio::test]
#[ignore]
async fn command_submit_share_password_correct() {
    let result = app_lib::commands::quark::submit_share_password(
        "abc123".to_string(),
        "stoken_value".to_string(),
        "1234".to_string(),
    )
    .await;
    assert!(result.is_ok(), "correct password should succeed");
}

#[tokio::test]
async fn command_submit_share_password_wrong() {
    let result = app_lib::commands::quark::submit_share_password(
        "abc123".to_string(),
        "stoken_value".to_string(),
        "wrong".to_string(),
    )
    .await;
    assert!(result.is_err(), "wrong password should fail");
}

#[tokio::test]
#[ignore]
async fn command_get_share_files_success() {
    let result = app_lib::commands::quark::get_share_files(
        "abc123".to_string(),
        "stoken".to_string(),
        "0".to_string(),
        1,
        50,
    )
    .await;
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore]
async fn command_transfer_files_success() {
    let result = app_lib::commands::quark::transfer_files(
        "abc123".to_string(),
        "stoken".to_string(),
        vec!["f1".to_string()],
        vec!["tok1".to_string()],
        "0".to_string(),
    )
    .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn command_transfer_files_empty_list() {
    let result = app_lib::commands::quark::transfer_files(
        "abc123".to_string(),
        "stoken".to_string(),
        vec![],
        vec![],
        "0".to_string(),
    )
    .await;
    assert!(result.is_err(), "empty file list should error");
}

#[tokio::test]
#[ignore]
async fn command_query_transfer_task() {
    let result = app_lib::commands::quark::query_transfer_task("task-001".to_string()).await;
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore]
async fn command_get_download_link() {
    let result = app_lib::commands::quark::get_download_link("fid1".to_string()).await;
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore]
async fn command_verify_credential() {
    let result = app_lib::commands::quark::verify_credential().await;
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore]
async fn command_get_user_directories() {
    let result = app_lib::commands::quark::get_user_directories("0".to_string()).await;
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore]
async fn command_add_download_task() {
    let result = app_lib::commands::rpc::add_download_task(
        "srv1".to_string(),
        vec!["http://example.com/file.mp4".to_string()],
        "file.mp4".to_string(),
        Some("D:\\Downloads".to_string()),
    )
    .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn command_test_rpc_connection() {
    let result =
        app_lib::commands::rpc::test_rpc_connection("http://localhost:6800".to_string(), None)
            .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn command_test_rpc_connection_with_token() {
    let result = app_lib::commands::rpc::test_rpc_connection(
        "http://localhost:6800".to_string(),
        Some("my_token".to_string()),
    )
    .await;
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore]
async fn command_query_rpc_task_status() {
    let result =
        app_lib::commands::rpc::query_rpc_task_status("srv1".to_string(), "gid123".to_string())
            .await;
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore]
async fn command_get_config() {
    let result = app_lib::commands::config::get_config().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn command_save_config() {
    let config = app_lib::models::config::AppConfig::default();
    let result = app_lib::commands::config::save_config(config).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn command_add_rpc_server() {
    let server = app_lib::models::config::RpcServer {
        id: "srv1".to_string(),
        name: "Aria2".to_string(),
        url: "http://localhost:6800".to_string(),
        token: None,
        downloader_type: app_lib::models::config::DownloaderType::Aria2,
        download_dir: None,
        is_default: true,
    };
    let result = app_lib::commands::config::add_rpc_server(server).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn command_update_rpc_server() {
    let server = app_lib::models::config::RpcServer {
        id: "srv1".to_string(),
        name: "Aria2 Updated".to_string(),
        url: "http://newhost:6800".to_string(),
        token: Some("token123".to_string()),
        downloader_type: app_lib::models::config::DownloaderType::Aria2,
        download_dir: Some("/downloads".to_string()),
        is_default: true,
    };
    let result = app_lib::commands::config::update_rpc_server(server).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn command_delete_rpc_server() {
    let result = app_lib::commands::config::delete_rpc_server("srv1".to_string()).await;
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore]
async fn command_set_default_rpc_server() {
    let server = app_lib::models::config::RpcServer {
        id: "test_srv_default".to_string(),
        name: "Test Default RPC".to_string(),
        url: "http://localhost:6800".to_string(),
        token: Some("token123".to_string()),
        downloader_type: app_lib::models::config::DownloaderType::Aria2,
        download_dir: Some("/downloads".to_string()),
        is_default: false,
    };
    let _ = app_lib::commands::config::add_rpc_server(server).await;

    let result =
        app_lib::commands::config::set_default_rpc_server("test_srv_default".to_string()).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn command_open_login_window() {
    let result = app_lib::commands::auth::open_login_window().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn command_verify_credential_status() {
    let result = app_lib::commands::auth::verify_credential_status().await;
    assert!(result.is_ok());
}
