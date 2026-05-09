use app_lib::services::rpc_client::{RpcClient, RpcTaskStatus};
use app_lib::models::config::DownloaderType;
use app_lib::error::AppError;
use pretty_assertions::assert_eq;

#[tokio::test]
async fn aria2_add_uri_returns_gid() {
    let mut server = mockito::Server::new_async().await;
    let mock = server.mock("POST", "/jsonrpc")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"1","jsonrpc":"2.0","result":"2089b05ecca3d829"}"#)
        .create_async()
        .await;

    let client = RpcClient::new(&server.url(), None);
    let result = client
        .add_uri(vec!["http://example.com/file.mp4"], "file.mp4", None)
        .await;
    assert!(result.is_ok(), "add_uri should succeed");
    assert_eq!(result.unwrap(), "2089b05ecca3d829");
}

#[tokio::test]
async fn aria2_add_uri_with_token() {
    let mut server = mockito::Server::new_async().await;
    let mock = server.mock("POST", "/jsonrpc")
        .match_body(mockito::Matcher::Regex(r#"token:my_secret_token"#.to_string()))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"1","jsonrpc":"2.0","result":"abc123gid"}"#)
        .create_async()
        .await;

    let client = RpcClient::new(&server.url(), Some("my_secret_token"));
    let result = client
        .add_uri(vec!["http://example.com/file.mp4"], "file.mp4", None)
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn aria2_add_uri_with_download_dir() {
    let mut server = mockito::Server::new_async().await;
    let mock = server.mock("POST", "/jsonrpc")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"1","jsonrpc":"2.0","result":"gid_with_dir"}"#)
        .create_async()
        .await;

    let client = RpcClient::new(&server.url(), None);
    let result = client
        .add_uri(vec!["http://example.com/file.mp4"], "file.mp4", Some("D:\\Downloads"))
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn aria2_add_multiple_urls() {
    let mut server = mockito::Server::new_async().await;
    let mock = server.mock("POST", "/jsonrpc")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"1","jsonrpc":"2.0","result":"multi_gid"}"#)
        .create_async()
        .await;

    let client = RpcClient::new(&server.url(), None);
    let result = client
        .add_uri(
            vec!["http://example.com/file1.mp4", "http://example.com/file2.mp4"],
            "files",
            None,
        )
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn aria2_connection_test_success() {
    let mut server = mockito::Server::new_async().await;
    let mock = server.mock("POST", "/jsonrpc")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"1","jsonrpc":"2.0","result":{"version":"1.37.0"}}"#)
        .create_async()
        .await;

    let client = RpcClient::new(&server.url(), None);
    let result = client.test_connection().await;
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[tokio::test]
async fn aria2_connection_test_failure() {
    let client = RpcClient::new("http://nonexistent-host.invalid:99999", None);
    let result = client.test_connection().await;
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[tokio::test]
async fn aria2_connection_test_unauthorized() {
    let mut server = mockito::Server::new_async().await;
    let mock = server.mock("POST", "/jsonrpc")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"1","jsonrpc":"2.0","error":{"code":1,"message":"Unauthorized"}}"#)
        .create_async()
        .await;

    let client = RpcClient::new(&server.url(), Some("wrong_token"));
    let result = client.test_connection().await;
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[tokio::test]
async fn aria2_query_task_status_active() {
    let mut server = mockito::Server::new_async().await;
    let mock = server.mock("POST", "/jsonrpc")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"1","jsonrpc":"2.0","result":{"status":"active","totalLength":"1073741824","completedLength":"536870912"}}"#)
        .create_async()
        .await;

    let client = RpcClient::new(&server.url(), None);
    let result = client.query_task_status("2089b05ecca3d829").await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), RpcTaskStatus::Active);
}

#[tokio::test]
async fn aria2_query_task_status_complete() {
    let mut server = mockito::Server::new_async().await;
    let mock = server.mock("POST", "/jsonrpc")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"1","jsonrpc":"2.0","result":{"status":"complete","totalLength":"1073741824","completedLength":"1073741824"}}"#)
        .create_async()
        .await;

    let client = RpcClient::new(&server.url(), None);
    let result = client.query_task_status("gid_complete").await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), RpcTaskStatus::Complete);
}

#[tokio::test]
async fn aria2_query_task_status_error() {
    let mut server = mockito::Server::new_async().await;
    let mock = server.mock("POST", "/jsonrpc")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"1","jsonrpc":"2.0","result":{"status":"error","errorCode":"1","errorMessage":"Download aborted"}}"#)
        .create_async()
        .await;

    let client = RpcClient::new(&server.url(), None);
    let result = client.query_task_status("gid_error").await;
    assert!(result.is_ok());
    match result.unwrap() {
        RpcTaskStatus::Error { message } => {
            assert_eq!(message, "Download aborted");
        }
        _ => panic!("expected Error status"),
    }
}

#[tokio::test]
async fn aria2_query_task_status_waiting() {
    let mut server = mockito::Server::new_async().await;
    let mock = server.mock("POST", "/jsonrpc")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"1","jsonrpc":"2.0","result":{"status":"waiting"}}"#)
        .create_async()
        .await;

    let client = RpcClient::new(&server.url(), None);
    let result = client.query_task_status("gid_waiting").await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), RpcTaskStatus::Waiting);
}

#[tokio::test]
async fn aria2_query_task_status_paused() {
    let mut server = mockito::Server::new_async().await;
    let mock = server.mock("POST", "/jsonrpc")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"1","jsonrpc":"2.0","result":{"status":"paused"}}"#)
        .create_async()
        .await;

    let client = RpcClient::new(&server.url(), None);
    let result = client.query_task_status("gid_paused").await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), RpcTaskStatus::Paused);
}

#[tokio::test]
async fn aria2_add_uri_server_error() {
    let mut server = mockito::Server::new_async().await;
    let mock = server.mock("POST", "/jsonrpc")
        .with_status(500)
        .with_body("Internal Server Error")
        .create_async()
        .await;

    let client = RpcClient::new(&server.url(), None);
    let result = client
        .add_uri(vec!["http://example.com/file.mp4"], "file.mp4", None)
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn rpc_client_with_bitcomet_type() {
    let mut server = mockito::Server::new_async().await;
    let mock = server.mock("POST", "/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"result":0}"#)
        .create_async()
        .await;

    let client = RpcClient::new(&server.url(), None)
        .with_downloader_type(DownloaderType::BitComet);
    let result = client.test_connection().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn rpc_client_connection_refused() {
    let client = RpcClient::new("http://127.0.0.1:1", None);
    let result = client.test_connection().await;
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[test]
fn rpc_task_status_variants_equality() {
    assert_eq!(RpcTaskStatus::Active, RpcTaskStatus::Active);
    assert_eq!(RpcTaskStatus::Complete, RpcTaskStatus::Complete);
    assert_ne!(RpcTaskStatus::Active, RpcTaskStatus::Waiting);
}

#[test]
fn rpc_task_status_error_contains_message() {
    let status = RpcTaskStatus::Error {
        message: "Connection refused".to_string(),
    };
    if let RpcTaskStatus::Error { message } = status {
        assert_eq!(message, "Connection refused");
    } else {
        panic!("expected Error variant");
    }
}
