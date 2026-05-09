use app_lib::services::rpc_client::{RpcClient, RpcTaskStatus};
use app_lib::models::config::DownloaderType;
use pretty_assertions::assert_eq;

fn create_test_server(body: &str) -> (mockito::Mock, String) {
    let mut server = mockito::Server::new();
    let _mock = server.mock("POST", "/jsonrpc")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(body);
    (mock, server.url())
}

#[test]
fn aria2_add_uri_returns_gid() {
    let (mock, url) = create_test_server(r#"{"id":"1","jsonrpc":"2.0","result":"2089b05ecca3d829"}"#);
    
    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(async {
        let client = RpcClient::new(&url, None);
        client.add_uri(vec!["http://example.com/file.mp4"], "file.mp4", None).await
    });
    
    assert!(result.is_ok(), "add_uri should succeed");
    assert_eq!(result.unwrap(), "2089b05ecca3d829");
}

#[test]
fn aria2_add_uri_with_token() {
    let (mock, url) = create_test_server(r#"{"id":"1","jsonrpc":"2.0","result":"abc123gid"}"#);
    
    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(async {
        let client = RpcClient::new(&url, Some("my_secret_token"));
        client.add_uri(vec!["http://example.com/file.mp4"], "file.mp4", None).await
    });
    
    assert!(result.is_ok());
}

#[test]
fn aria2_add_uri_with_download_dir() {
    let (mock, url) = create_test_server(r#"{"id":"1","jsonrpc":"2.0","result":"gid_with_dir"}"#);
    
    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(async {
        let client = RpcClient::new(&url, None);
        client.add_uri(vec!["http://example.com/file.mp4"], "file.mp4", Some("D:\\Downloads")).await
    });
    
    assert!(result.is_ok());
}

#[test]
fn aria2_add_multiple_urls() {
    let (mock, url) = create_test_server(r#"{"id":"1","jsonrpc":"2.0","result":"multi_gid"}"#);
    
    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(async {
        let client = RpcClient::new(&url, None);
        client.add_uri(
            vec!["http://example.com/file1.mp4", "http://example.com/file2.mp4"],
            "files",
            None,
        ).await
    });
    
    assert!(result.is_ok());
}

#[test]
fn aria2_connection_test_success() {
    let (mock, url) = create_test_server(r#"{"id":"1","jsonrpc":"2.0","result":{"version":"1.37.0"}}"#);
    
    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(async {
        let client = RpcClient::new(&url, None);
        client.test_connection().await
    });
    
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
fn aria2_connection_test_failure() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(async {
        let client = RpcClient::new("http://nonexistent-host.invalid:99999", None);
        client.test_connection().await
    });
    
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[test]
fn aria2_connection_test_unauthorized() {
    let (mock, url) = create_test_server(r#"{"id":"1","jsonrpc":"2.0","error":{"code":1,"message":"Unauthorized"}}"#);
    
    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(async {
        let client = RpcClient::new(&url, Some("wrong_token"));
        client.test_connection().await
    });
    
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[test]
fn aria2_query_task_status_active() {
    let (mock, url) = create_test_server(r#"{"id":"1","jsonrpc":"2.0","result":{"status":"active","totalLength":"1073741824","completedLength":"536870912"}}"#);
    
    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(async {
        let client = RpcClient::new(&url, None);
        client.query_task_status("2089b05ecca3d829").await
    });
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), RpcTaskStatus::Active);
}

#[test]
fn aria2_query_task_status_complete() {
    let (mock, url) = create_test_server(r#"{"id":"1","jsonrpc":"2.0","result":{"status":"complete","totalLength":"1073741824","completedLength":"1073741824"}}"#);
    
    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(async {
        let client = RpcClient::new(&url, None);
        client.query_task_status("gid_complete").await
    });
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), RpcTaskStatus::Complete);
}

#[test]
fn aria2_query_task_status_error() {
    let (mock, url) = create_test_server(r#"{"id":"1","jsonrpc":"2.0","result":{"status":"error","errorCode":"1","errorMessage":"Download aborted"}}"#);
    
    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(async {
        let client = RpcClient::new(&url, None);
        client.query_task_status("gid_error").await
    });
    
    assert!(result.is_ok());
    match result.unwrap() {
        RpcTaskStatus::Error { message } => {
            assert_eq!(message, "Download aborted");
        }
        _ => panic!("expected Error status"),
    }
}

#[test]
fn aria2_query_task_status_waiting() {
    let (mock, url) = create_test_server(r#"{"id":"1","jsonrpc":"2.0","result":{"status":"waiting"}}"#);
    
    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(async {
        let client = RpcClient::new(&url, None);
        client.query_task_status("gid_waiting").await
    });
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), RpcTaskStatus::Waiting);
}

#[test]
fn aria2_query_task_status_paused() {
    let (mock, url) = create_test_server(r#"{"id":"1","jsonrpc":"2.0","result":{"status":"paused"}}"#);
    
    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(async {
        let client = RpcClient::new(&url, None);
        client.query_task_status("gid_paused").await
    });
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), RpcTaskStatus::Paused);
}

#[test]
fn aria2_add_uri_server_error() {
    let mut server = mockito::Server::new();
    let _mock = server.mock("POST", "/jsonrpc")
        .with_status(500)
        .with_body("Internal Server Error");
    
    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(async {
        let client = RpcClient::new(&server.url(), None);
        client.add_uri(vec!["http://example.com/file.mp4"], "file.mp4", None).await
    });
    
    assert!(result.is_err());
}

#[test]
fn rpc_client_with_bitcomet_type() {
    let mut server = mockito::Server::new();
    let _mock = server.mock("POST", "/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"result":0}"#);
    
    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(async {
        let client = RpcClient::new(&server.url(), None)
            .with_downloader_type(DownloaderType::BitComet);
        client.test_connection().await
    });
    
    assert!(result.is_ok());
}

#[test]
fn rpc_client_connection_refused() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(async {
        let client = RpcClient::new("http://127.0.0.1:1", None);
        client.test_connection().await
    });
    
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
