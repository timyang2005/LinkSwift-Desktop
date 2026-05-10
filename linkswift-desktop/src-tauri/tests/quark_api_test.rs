use app_lib::services::quark_api::{QuarkApi, TransferTaskStatus};
use pretty_assertions::assert_eq;

#[test]
fn parse_share_url_valid_standard() {
    let url = "https://pan.quark.cn/s/abc123";
    let result = QuarkApi::parse_share_url(url);
    assert!(result.is_ok(), "valid URL should parse successfully");
    assert_eq!(result.unwrap(), "abc123");
}

#[test]
fn parse_share_url_valid_with_trailing_slash() {
    let url = "https://pan.quark.cn/s/abc123/";
    let result = QuarkApi::parse_share_url(url);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "abc123");
}

#[test]
fn parse_share_url_valid_with_query_params() {
    let url = "https://pan.quark.cn/s/abc123?ref=some_ref";
    let result = QuarkApi::parse_share_url(url);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "abc123");
}

#[test]
fn parse_share_url_invalid_domain() {
    let url = "https://example.com/s/abc123";
    let result = QuarkApi::parse_share_url(url);
    assert!(result.is_err(), "non-quark URL should fail");
}

#[test]
fn parse_share_url_invalid_format() {
    let url = "not-a-url";
    let result = QuarkApi::parse_share_url(url);
    assert!(result.is_err(), "malformed URL should fail");
}

#[test]
fn parse_share_url_missing_share_id() {
    let url = "https://pan.quark.cn/s/";
    let result = QuarkApi::parse_share_url(url);
    assert!(result.is_err(), "URL without share ID should fail");
}

#[test]
fn parse_share_url_empty_string() {
    let url = "";
    let result = QuarkApi::parse_share_url(url);
    assert!(result.is_err(), "empty string should fail");
}

#[test]
fn parse_share_url_http_redirect() {
    let url = "http://pan.quark.cn/s/abc123";
    let result = QuarkApi::parse_share_url(url);
    assert!(result.is_ok(), "http URL should also be accepted");
}

#[tokio::test]
async fn get_share_token_success() {
    let mut server = mockito::Server::new_async().await;
    let _mock = server
        .mock("POST", "/1/clouddrive/share/sharepage/token")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"data":{"stoken":"test_stoken_123"}}"#)
        .create_async()
        .await;

    let api = QuarkApi::new(&server.url());
    let result = api.get_share_token("abc123", None).await;
    assert!(result.is_ok(), "get_share_token should succeed");
    assert_eq!(result.unwrap(), "test_stoken_123");
    _mock.assert_async().await;
}

#[tokio::test]
async fn get_share_token_with_password() {
    let mut server = mockito::Server::new_async().await;
    let _mock = server
        .mock("POST", "/1/clouddrive/share/sharepage/token")
        .match_body(mockito::Matcher::JsonString(
            r#"{"pwd_id":"abc123","passcode":"1234"}"#.to_string(),
        ))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"data":{"stoken":"pwd_stoken"}}"#)
        .create_async()
        .await;

    let api = QuarkApi::new(&server.url());
    let result = api.get_share_token("abc123", Some("1234")).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "pwd_stoken");
}

#[tokio::test]
async fn get_share_token_expired_link() {
    let mut server = mockito::Server::new_async().await;
    let _mock = server
        .mock("POST", "/1/clouddrive/share/sharepage/token")
        .with_status(403)
        .with_header("content-type", "application/json")
        .with_body(r#"{"code":403,"message":"分享已失效"}"#)
        .create_async()
        .await;

    let api = QuarkApi::new(&server.url());
    let result = api.get_share_token("expired_id", None).await;
    assert!(result.is_err(), "expired share should return error");
}

#[tokio::test]
async fn get_share_token_wrong_password() {
    let mut server = mockito::Server::new_async().await;
    let _mock = server
        .mock("POST", "/1/clouddrive/share/sharepage/token")
        .with_status(400)
        .with_header("content-type", "application/json")
        .with_body(r#"{"code":400,"message":"提取码错误"}"#)
        .create_async()
        .await;

    let api = QuarkApi::new(&server.url());
    let result = api.get_share_token("abc123", Some("wrong")).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn get_share_files_returns_file_list() {
    let mut server = mockito::Server::new_async().await;
    let _mock = server.mock("GET", mockito::Matcher::Regex(r"/1/clouddrive/share/sharepage/detail.*".to_string()))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{
            "data": {
                "list": [
                    {"fid":"f1","name":"movie.mp4","is_folder":false,"size":1073741824,"pdir_fid":"0","selected":false},
                    {"fid":"f2","name":"docs","is_folder":true,"size":0,"pdir_fid":"0","selected":false}
                ],
                "total_count": 2,
                "page": 1,
                "page_size": 50,
                "has_more": false
            }
        }"#)
        .create_async()
        .await;

    let api = QuarkApi::new(&server.url());
    let result = api.get_share_files("abc123", "stoken", "0", 1, 50).await;
    assert!(result.is_ok());
    let files = result.unwrap();
    assert_eq!(files.items.len(), 2);
    assert_eq!(files.items[0].name, "movie.mp4");
    assert!(!files.items[0].is_folder);
    assert!(files.items[1].is_folder);
}

#[tokio::test]
async fn get_share_files_sub_directory() {
    let mut server = mockito::Server::new_async().await;
    let _mock = server.mock("GET", mockito::Matcher::Regex(r"/1/clouddrive/share/sharepage/detail.*".to_string()))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{
            "data": {
                "list": [
                    {"fid":"f3","name":"subfile.txt","is_folder":false,"size":256,"pdir_fid":"f2","selected":false}
                ],
                "total_count": 1,
                "page": 1,
                "page_size": 50,
                "has_more": false
            }
        }"#)
        .create_async()
        .await;

    let api = QuarkApi::new(&server.url());
    let result = api.get_share_files("abc123", "stoken", "f2", 1, 50).await;
    assert!(result.is_ok());
    let files = result.unwrap();
    assert_eq!(files.items.len(), 1);
    assert_eq!(files.items[0].pdir_fid, "f2");
}

#[tokio::test]
async fn get_share_files_paginated_has_more() {
    let mut server = mockito::Server::new_async().await;
    let _mock = server
        .mock(
            "GET",
            mockito::Matcher::Regex(r"/1/clouddrive/share/sharepage/detail.*".to_string()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{
            "data": {
                "list": [],
                "total_count": 100,
                "page": 1,
                "page_size": 50,
                "has_more": true
            }
        }"#,
        )
        .create_async()
        .await;

    let api = QuarkApi::new(&server.url());
    let result = api.get_share_files("abc123", "stoken", "0", 1, 50).await;
    assert!(result.is_ok());
    let files = result.unwrap();
    assert!(files.has_more);
    assert_eq!(files.total_count, 100);
}

#[tokio::test]
async fn get_share_files_unauthorized() {
    let mut server = mockito::Server::new_async().await;
    let _mock = server
        .mock(
            "GET",
            mockito::Matcher::Regex(r"/1/clouddrive/share/sharepage/detail.*".to_string()),
        )
        .with_status(401)
        .with_body(r#"{"code":401,"message":"未授权"}"#)
        .create_async()
        .await;

    let api = QuarkApi::new(&server.url());
    let result = api.get_share_files("abc123", "stoken", "0", 1, 50).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn transfer_files_returns_task_id() {
    let mut server = mockito::Server::new_async().await;
    let _mock = server
        .mock("POST", "/1/clouddrive/share/sharepage/save")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"data":{"task_id":"task-001"}}"#)
        .create_async()
        .await;

    let api = QuarkApi::new(&server.url());
    let result = api
        .transfer_files(
            "abc123",
            "stoken",
            &["f1".to_string()],
            &["tok1".to_string()],
            "0",
        )
        .await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "task-001");
}

#[tokio::test]
async fn transfer_files_multiple_files() {
    let mut server = mockito::Server::new_async().await;
    let _mock = server
        .mock("POST", "/1/clouddrive/share/sharepage/save")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"data":{"task_id":"task-002"}}"#)
        .create_async()
        .await;

    let api = QuarkApi::new(&server.url());
    let result = api
        .transfer_files(
            "abc123",
            "stoken",
            &["f1".to_string(), "f2".to_string()],
            &["tok1".to_string(), "tok2".to_string()],
            "target_dir",
        )
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn transfer_files_empty_fid_list_returns_error() {
    let api = QuarkApi::new("http://unused.example.com");
    let result = api.transfer_files("abc123", "stoken", &[], &[], "0").await;
    assert!(result.is_err(), "empty fid list should return error");
}

#[tokio::test]
async fn transfer_files_server_error() {
    let mut server = mockito::Server::new_async().await;
    let _mock = server
        .mock("POST", "/1/clouddrive/share/sharepage/save")
        .with_status(500)
        .with_body(r#"{"code":500,"message":"服务器内部错误"}"#)
        .create_async()
        .await;

    let api = QuarkApi::new(&server.url());
    let result = api
        .transfer_files(
            "abc123",
            "stoken",
            &["f1".to_string()],
            &["tok1".to_string()],
            "0",
        )
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn query_transfer_task_completed() {
    let mut server = mockito::Server::new_async().await;
    let _mock = server
        .mock(
            "GET",
            mockito::Matcher::Regex(r"/1/clouddrive/task.*".to_string()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"data":{"status":2,"new_fids":["new_f1","new_f2"]}}"#)
        .create_async()
        .await;

    let api = QuarkApi::new(&server.url());
    let result = api.query_transfer_task("task-001").await;
    assert!(result.is_ok());
    match result.unwrap() {
        TransferTaskStatus::Completed { new_fids } => {
            assert_eq!(new_fids.len(), 2);
        }
        _ => panic!("expected Completed status"),
    }
}

#[tokio::test]
async fn query_transfer_task_in_progress() {
    let mut server = mockito::Server::new_async().await;
    let _mock = server
        .mock(
            "GET",
            mockito::Matcher::Regex(r"/1/clouddrive/task.*".to_string()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"data":{"status":1,"progress":67}}"#)
        .create_async()
        .await;

    let api = QuarkApi::new(&server.url());
    let result = api.query_transfer_task("task-001").await;
    assert!(result.is_ok());
    match result.unwrap() {
        TransferTaskStatus::Running { progress } => {
            assert_eq!(progress, 67);
        }
        _ => panic!("expected Running status"),
    }
}

#[tokio::test]
async fn query_transfer_task_failed() {
    let mut server = mockito::Server::new_async().await;
    let _mock = server
        .mock(
            "GET",
            mockito::Matcher::Regex(r"/1/clouddrive/task.*".to_string()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"data":{"status":3,"failed_reason":"空间不足"}}"#)
        .create_async()
        .await;

    let api = QuarkApi::new(&server.url());
    let result = api.query_transfer_task("task-001").await;
    assert!(result.is_ok());
    match result.unwrap() {
        TransferTaskStatus::Failed { reason } => {
            assert_eq!(reason, "空间不足");
        }
        _ => panic!("expected Failed status"),
    }
}

#[tokio::test]
async fn get_download_link_returns_url() {
    let mut server = mockito::Server::new_async().await;
    let _mock = server
        .mock("POST", "/1/clouddrive/file/download")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{
            "data": [{
                "fid": "fid1",
                "name": "movie.mp4",
                "download_url": "https://dl.quark.cn/xxx",
                "size": 1073741824,
                "md5": "abc123",
                "expires_in": 3600
            }]
        }"#,
        )
        .create_async()
        .await;

    let api = QuarkApi::new(&server.url());
    let result = api.get_download_link("fid1").await;
    assert!(result.is_ok());
    let link = result.unwrap();
    assert_eq!(link.fid, "fid1");
    assert_eq!(link.name, "movie.mp4");
    assert!(link.url.starts_with("https://"));
    assert_eq!(link.expires_in, 3600);
}

#[tokio::test]
async fn get_download_link_invalid_fid() {
    let mut server = mockito::Server::new_async().await;
    let _mock = server
        .mock("POST", "/1/clouddrive/file/download")
        .with_status(404)
        .with_body(r#"{"code":404,"message":"文件不存在"}"#)
        .create_async()
        .await;

    let api = QuarkApi::new(&server.url());
    let result = api.get_download_link("invalid_fid").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn verify_credential_valid() {
    let mut server = mockito::Server::new_async().await;
    let _mock = server
        .mock(
            "GET",
            mockito::Matcher::Regex(r"/1/clouddrive/file/sort.*".to_string()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"data":{"list":[]}}"#)
        .create_async()
        .await;

    let api = QuarkApi::new(&server.url()).with_cookie("valid_cookie_value");
    let result = api.verify_credential().await;
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[tokio::test]
async fn verify_credential_expired() {
    let mut server = mockito::Server::new_async().await;
    let _mock = server
        .mock(
            "GET",
            mockito::Matcher::Regex(r"/1/clouddrive/file/sort.*".to_string()),
        )
        .with_status(401)
        .with_body(r#"{"code":401,"message":"未授权"}"#)
        .create_async()
        .await;

    let api = QuarkApi::new(&server.url()).with_cookie("expired_cookie");
    let result = api.verify_credential().await;
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[tokio::test]
async fn get_user_directories_returns_list() {
    let mut server = mockito::Server::new_async().await;
    let _mock = server.mock("GET", mockito::Matcher::Regex(r"/1/clouddrive/file/sort.*".to_string()))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{
            "data": {
                "list": [
                    {"fid":"dir1","name":"文档","is_folder":true,"size":0,"pdir_fid":"0","selected":false},
                    {"fid":"dir2","name":"视频","is_folder":true,"size":0,"pdir_fid":"0","selected":false}
                ]
            }
        }"#)
        .create_async()
        .await;

    let api = QuarkApi::new(&server.url()).with_cookie("valid_cookie");
    let result = api.get_user_directories("0").await;
    assert!(result.is_ok());
    let dirs = result.unwrap();
    assert_eq!(dirs.len(), 2);
    assert!(dirs[0].is_folder);
    assert!(dirs[1].is_folder);
}

#[tokio::test]
async fn network_error_returns_app_error() {
    let api = QuarkApi::new("http://nonexistent-host.invalid:99999");
    let result = api.get_share_token("abc", None).await;
    assert!(result.is_err());
}
