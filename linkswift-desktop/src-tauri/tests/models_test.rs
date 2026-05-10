use app_lib::models::config::{
    AppConfig, CredentialConfig, DownloaderType, ProxyConfig, RpcServer, Theme,
};
use app_lib::models::file::{FileItem, PaginatedFiles};
use app_lib::models::share::{ShareInfo, ShareTokenRequest, ShareTokenResponse};
use app_lib::models::task::{DownloadLink, DownloadTask, TaskStatus};
use pretty_assertions::assert_eq;

#[test]
fn file_item_deserialize_from_json() {
    let json = r#"{
        "fid": "abc123",
        "name": "test.mp4",
        "is_folder": false,
        "size": 1024,
        "pdir_fid": "parent0",
        "mime_type": "video/mp4",
        "file_icon": "video",
        "created_at": 1700000000,
        "updated_at": 1700000100,
        "share_fid_token": "token_xyz",
        "selected": false
    }"#;
    let item: FileItem = serde_json::from_str(json).unwrap();
    assert_eq!(item.fid, "abc123");
    assert_eq!(item.name, "test.mp4");
    assert!(!item.is_folder);
    assert_eq!(item.size, 1024);
    assert_eq!(item.pdir_fid, "parent0");
    assert_eq!(item.mime_type, Some("video/mp4".to_string()));
    assert_eq!(item.share_fid_token, Some("token_xyz".to_string()));
    assert!(!item.selected);
}

#[test]
fn file_item_folder_has_zero_size() {
    let json = r#"{
        "fid": "dir1",
        "name": "folder",
        "is_folder": true,
        "size": 0,
        "pdir_fid": "0",
        "selected": false
    }"#;
    let item: FileItem = serde_json::from_str(json).unwrap();
    assert!(item.is_folder);
    assert_eq!(item.size, 0);
    assert!(item.mime_type.is_none());
}

#[test]
fn file_item_optional_fields_default_to_none() {
    let json = r#"{
        "fid": "f1",
        "name": "doc.txt",
        "is_folder": false,
        "size": 256,
        "pdir_fid": "0",
        "selected": false
    }"#;
    let item: FileItem = serde_json::from_str(json).unwrap();
    assert!(item.mime_type.is_none());
    assert!(item.file_icon.is_none());
    assert!(item.created_at.is_none());
    assert!(item.updated_at.is_none());
    assert!(item.share_fid_token.is_none());
}

#[test]
fn file_item_selected_defaults_to_false() {
    let json = r#"{
        "fid": "f2",
        "name": "image.png",
        "is_folder": false,
        "size": 512,
        "pdir_fid": "0"
    }"#;
    let item: FileItem = serde_json::from_str(json).unwrap();
    assert!(!item.selected);
}

#[test]
fn file_item_serialize_deserialize_roundtrip() {
    let item = FileItem {
        fid: "fid_round".to_string(),
        name: "roundtrip.mkv".to_string(),
        is_folder: false,
        size: 999999,
        pdir_fid: "pdir".to_string(),
        mime_type: Some("video/x-matroska".to_string()),
        file_icon: Some("video".to_string()),
        created_at: Some(1700000000),
        updated_at: Some(1700000100),
        share_fid_token: Some("tok".to_string()),
        selected: true,
    };
    let json = serde_json::to_string(&item).unwrap();
    let parsed: FileItem = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed, item);
}

#[test]
fn paginated_files_deserialize() {
    let json = r#"{
        "items": [
            {"fid":"f1","name":"a.txt","is_folder":false,"size":100,"pdir_fid":"0","selected":false},
            {"fid":"f2","name":"b.txt","is_folder":false,"size":200,"pdir_fid":"0","selected":false}
        ],
        "total_count": 2,
        "page": 1,
        "page_size": 50,
        "has_more": false
    }"#;
    let pag: PaginatedFiles = serde_json::from_str(json).unwrap();
    assert_eq!(pag.items.len(), 2);
    assert_eq!(pag.total_count, 2);
    assert!(!pag.has_more);
}

#[test]
fn share_info_deserialize() {
    let json = r#"{
        "pwd_id": "abc123",
        "stoken": "stoken_value",
        "title": "测试分享",
        "has_password": true,
        "files": []
    }"#;
    let info: ShareInfo = serde_json::from_str(json).unwrap();
    assert_eq!(info.pwd_id, "abc123");
    assert_eq!(info.stoken, "stoken_value");
    assert_eq!(info.title, "测试分享");
    assert!(info.has_password);
    assert!(info.files.is_empty());
}

#[test]
fn share_info_without_password() {
    let json = r#"{
        "pwd_id": "xyz789",
        "stoken": "tok2",
        "title": "公开分享",
        "has_password": false,
        "files": [
            {"fid":"f1","name":"file.mp4","is_folder":false,"size":1024,"pdir_fid":"0","selected":false}
        ]
    }"#;
    let info: ShareInfo = serde_json::from_str(json).unwrap();
    assert!(!info.has_password);
    assert_eq!(info.files.len(), 1);
}

#[test]
fn share_token_request_serialize() {
    let req = ShareTokenRequest {
        pwd_id: "abc".to_string(),
        passcode: Some("1234".to_string()),
    };
    let json = serde_json::to_string(&req).unwrap();
    assert!(json.contains("abc"));
    assert!(json.contains("1234"));
}

#[test]
fn share_token_request_without_passcode() {
    let req = ShareTokenRequest {
        pwd_id: "abc".to_string(),
        passcode: None,
    };
    let json = serde_json::to_string(&req).unwrap();
    assert!(json.contains("abc"));
}

#[test]
fn share_token_response_deserialize() {
    let json = r#"{"stoken":"my_stoken_123"}"#;
    let resp: ShareTokenResponse = serde_json::from_str(json).unwrap();
    assert_eq!(resp.stoken, "my_stoken_123");
}

#[test]
fn app_config_default_values() {
    let config = AppConfig::default();
    assert!(config.is_first_run);
    assert!(config.credential.encrypted_cookie.is_empty());
    assert!(!config.credential.is_valid);
    assert_eq!(config.credential.last_verified, 0);
    assert_eq!(config.credential.remind_before_expire_days, 7);
    assert!(config.rpc_servers.is_empty());
    assert_eq!(config.default_rpc_index, 0);
    assert_eq!(config.theme, Theme::System);
    assert!(config.proxy.is_none());
    assert_eq!(config.retry_count, 3);
}

#[test]
fn app_config_serialize_deserialize_roundtrip() {
    let config = AppConfig {
        credential: CredentialConfig {
            encrypted_cookie: "enc_data".to_string(),
            last_verified: 1700000000,
            is_valid: true,
            remind_before_expire_days: 5,
        },
        rpc_servers: vec![RpcServer {
            id: "srv1".to_string(),
            name: "Aria2".to_string(),
            url: "http://localhost:6800".to_string(),
            token: Some("secret".to_string()),
            downloader_type: DownloaderType::Aria2,
            download_dir: Some("/downloads".to_string()),
            is_default: true,
        }],
        default_rpc_index: 0,
        theme: Theme::Dark,
        proxy: Some(ProxyConfig {
            url: "http://127.0.0.1:7890".to_string(),
            username: Some("user".to_string()),
            password: Some("pass".to_string()),
        }),
        is_first_run: false,
        retry_count: 5,
    };
    let json = serde_json::to_string(&config).unwrap();
    let parsed: AppConfig = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed, config);
}

#[test]
fn rpc_server_fields() {
    let server = RpcServer {
        id: "id1".to_string(),
        name: "Aria2".to_string(),
        url: "http://localhost:6800".to_string(),
        token: None,
        downloader_type: DownloaderType::Aria2,
        download_dir: None,
        is_default: true,
    };
    assert_eq!(server.id, "id1");
    assert_eq!(server.name, "Aria2");
    assert!(server.token.is_none());
    assert_eq!(server.downloader_type, DownloaderType::Aria2);
}

#[test]
fn downloader_type_variants_serialize() {
    let types = vec![
        DownloaderType::Aria2,
        DownloaderType::BitComet,
        DownloaderType::ABDownloadManager,
        DownloaderType::Custom,
    ];
    for dt in &types {
        let json = serde_json::to_string(dt).unwrap();
        let parsed: DownloaderType = serde_json::from_str(&json).unwrap();
        assert_eq!(&parsed, dt);
    }
}

#[test]
fn theme_variants_serialize() {
    let themes = vec![Theme::Light, Theme::Dark, Theme::System];
    for t in &themes {
        let json = serde_json::to_string(t).unwrap();
        let parsed: Theme = serde_json::from_str(&json).unwrap();
        assert_eq!(&parsed, t);
    }
}

#[test]
fn proxy_config_with_auth() {
    let proxy = ProxyConfig {
        url: "http://proxy:8080".to_string(),
        username: Some("admin".to_string()),
        password: Some("secret".to_string()),
    };
    let json = serde_json::to_string(&proxy).unwrap();
    let parsed: ProxyConfig = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed, proxy);
}

#[test]
fn proxy_config_without_auth() {
    let proxy = ProxyConfig {
        url: "http://proxy:8080".to_string(),
        username: None,
        password: None,
    };
    let json = serde_json::to_string(&proxy).unwrap();
    let parsed: ProxyConfig = serde_json::from_str(&json).unwrap();
    assert!(parsed.username.is_none());
    assert!(parsed.password.is_none());
}

#[test]
fn task_status_pending_serializes() {
    let status = TaskStatus::Pending;
    let json = serde_json::to_string(&status).unwrap();
    let parsed: TaskStatus = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed, TaskStatus::Pending);
}

#[test]
fn task_status_transferring_with_progress() {
    let status = TaskStatus::Transferring { progress: 67 };
    let json = serde_json::to_string(&status).unwrap();
    assert!(json.contains("67"));
    let parsed: TaskStatus = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed, TaskStatus::Transferring { progress: 67 });
}

#[test]
fn task_status_failed_with_reason() {
    let status = TaskStatus::Failed {
        reason: "Cookie expired".to_string(),
    };
    let json = serde_json::to_string(&status).unwrap();
    assert!(json.contains("Cookie expired"));
    let parsed: TaskStatus = serde_json::from_str(&json).unwrap();
    assert_eq!(
        parsed,
        TaskStatus::Failed {
            reason: "Cookie expired".to_string(),
        }
    );
}

#[test]
fn task_status_all_variants_roundtrip() {
    let statuses = vec![
        TaskStatus::Pending,
        TaskStatus::Parsing,
        TaskStatus::Transferring { progress: 50 },
        TaskStatus::FetchingLink,
        TaskStatus::Pushing,
        TaskStatus::Completed,
        TaskStatus::Failed {
            reason: "error".to_string(),
        },
        TaskStatus::Cancelled,
    ];
    for status in &statuses {
        let json = serde_json::to_string(status).unwrap();
        let parsed: TaskStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(&parsed, status);
    }
}

#[test]
fn download_link_deserialize() {
    let json = r#"{
        "fid": "fid1",
        "name": "movie.mp4",
        "url": "https://dl.quark.cn/xxx",
        "size": 1073741824,
        "md5": "abc123def",
        "expires_in": 3600
    }"#;
    let link: DownloadLink = serde_json::from_str(json).unwrap();
    assert_eq!(link.fid, "fid1");
    assert_eq!(link.name, "movie.mp4");
    assert!(link.url.starts_with("https://"));
    assert_eq!(link.size, 1073741824);
    assert_eq!(link.md5, Some("abc123def".to_string()));
    assert_eq!(link.expires_in, 3600);
}

#[test]
fn download_link_without_md5() {
    let json = r#"{
        "fid": "fid2",
        "name": "doc.pdf",
        "url": "https://dl.quark.cn/yyy",
        "size": 2048,
        "md5": null,
        "expires_in": 1800
    }"#;
    let link: DownloadLink = serde_json::from_str(json).unwrap();
    assert!(link.md5.is_none());
}

#[test]
fn download_task_full_roundtrip() {
    let task = DownloadTask {
        id: "task-001".to_string(),
        share_url: "https://pan.quark.cn/s/abc".to_string(),
        files: vec![FileItem {
            fid: "f1".to_string(),
            name: "test.mp4".to_string(),
            is_folder: false,
            size: 1024,
            pdir_fid: "0".to_string(),
            mime_type: None,
            file_icon: None,
            created_at: None,
            updated_at: None,
            share_fid_token: Some("tok".to_string()),
            selected: true,
        }],
        status: TaskStatus::Transferring { progress: 42 },
        target_dir: "0".to_string(),
        rpc_server_id: "srv1".to_string(),
        created_at: 1700000000,
        error_message: None,
        retry_count: 0,
    };
    let json = serde_json::to_string(&task).unwrap();
    let parsed: DownloadTask = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.id, "task-001");
    assert_eq!(parsed.files.len(), 1);
    assert_eq!(parsed.status, TaskStatus::Transferring { progress: 42 });
}

#[test]
fn download_task_failed_with_error_message() {
    let task = DownloadTask {
        id: "task-002".to_string(),
        share_url: "https://pan.quark.cn/s/xyz".to_string(),
        files: vec![],
        status: TaskStatus::Failed {
            reason: "转存失败: 空间不足".to_string(),
        },
        target_dir: "0".to_string(),
        rpc_server_id: "srv1".to_string(),
        created_at: 1700000000,
        error_message: Some("转存失败: 空间不足".to_string()),
        retry_count: 3,
    };
    let json = serde_json::to_string(&task).unwrap();
    let parsed: DownloadTask = serde_json::from_str(&json).unwrap();
    assert!(matches!(parsed.status, TaskStatus::Failed { .. }));
    assert_eq!(parsed.retry_count, 3);
}
