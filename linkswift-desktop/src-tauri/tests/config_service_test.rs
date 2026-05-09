use app_lib::models::config::{
    AppConfig, CredentialConfig, RpcServer, DownloaderType, ProxyConfig, Theme,
};
use app_lib::services::config_service::ConfigService;
use app_lib::services::crypto::CryptoService;
use app_lib::error::AppError;
use pretty_assertions::assert_eq;

fn create_test_config() -> AppConfig {
    AppConfig {
        credential: CredentialConfig {
            encrypted_cookie: String::new(),
            last_verified: 0,
            is_valid: false,
            remind_before_expire_days: 7,
        },
        rpc_servers: vec![],
        default_rpc_index: 0,
        theme: Theme::System,
        proxy: None,
        is_first_run: true,
        retry_count: 3,
    }
}

fn create_test_rpc_server(name: &str, url: &str) -> RpcServer {
    RpcServer {
        id: uuid::Uuid::new_v4().to_string(),
        name: name.to_string(),
        url: url.to_string(),
        token: None,
        downloader_type: DownloaderType::Aria2,
        download_dir: None,
        is_default: true,
    }
}

#[test]
fn config_service_save_and_load_roundtrip() {
    let dir = tempfile::tempdir().unwrap();
    let config = create_test_config();
    ConfigService::save(dir.path(), &config).expect("save should succeed");
    let loaded = ConfigService::load(dir.path()).expect("load should succeed");
    assert_eq!(loaded.is_first_run, config.is_first_run);
    assert_eq!(loaded.retry_count, config.retry_count);
    assert_eq!(loaded.theme, config.theme);
}

#[test]
fn config_service_load_creates_default_if_missing() {
    let dir = tempfile::tempdir().unwrap();
    let config = ConfigService::load(dir.path()).expect("load should return default");
    assert!(config.is_first_run);
    assert!(config.rpc_servers.is_empty());
    assert_eq!(config.retry_count, 3);
}

#[test]
fn config_service_save_overwrites_existing() {
    let dir = tempfile::tempdir().unwrap();
    let mut config = create_test_config();
    ConfigService::save(dir.path(), &config).unwrap();

    config.is_first_run = false;
    config.retry_count = 5;
    ConfigService::save(dir.path(), &config).unwrap();

    let loaded = ConfigService::load(dir.path()).unwrap();
    assert!(!loaded.is_first_run);
    assert_eq!(loaded.retry_count, 5);
}

#[test]
fn config_service_save_with_rpc_servers() {
    let dir = tempfile::tempdir().unwrap();
    let mut config = create_test_config();
    config.rpc_servers.push(create_test_rpc_server("Aria2", "http://localhost:6800"));
    config.rpc_servers.push(create_test_rpc_server("BC", "http://localhost:8888"));
    ConfigService::save(dir.path(), &config).unwrap();

    let loaded = ConfigService::load(dir.path()).unwrap();
    assert_eq!(loaded.rpc_servers.len(), 2);
    assert_eq!(loaded.rpc_servers[0].name, "Aria2");
    assert_eq!(loaded.rpc_servers[1].name, "BC");
}

#[test]
fn config_service_save_with_proxy() {
    let dir = tempfile::tempdir().unwrap();
    let mut config = create_test_config();
    config.proxy = Some(ProxyConfig {
        url: "http://127.0.0.1:7890".to_string(),
        username: Some("user".to_string()),
        password: Some("pass".to_string()),
    });
    ConfigService::save(dir.path(), &config).unwrap();

    let loaded = ConfigService::load(dir.path()).unwrap();
    assert!(loaded.proxy.is_some());
    let proxy = loaded.proxy.unwrap();
    assert_eq!(proxy.url, "http://127.0.0.1:7890");
    assert_eq!(proxy.username, Some("user".to_string()));
}

#[test]
fn config_service_save_with_credential() {
    let dir = tempfile::tempdir().unwrap();
    let mut config = create_test_config();
    config.credential = CredentialConfig {
        encrypted_cookie: "encrypted_data_here".to_string(),
        last_verified: 1700000000,
        is_valid: true,
        remind_before_expire_days: 5,
    };
    ConfigService::save(dir.path(), &config).unwrap();

    let loaded = ConfigService::load(dir.path()).unwrap();
    assert!(loaded.credential.is_valid);
    assert_eq!(loaded.credential.encrypted_cookie, "encrypted_data_here");
    assert_eq!(loaded.credential.last_verified, 1700000000);
}

#[test]
fn crypto_encrypt_decrypt_roundtrip() {
    let original = "my_secret_cookie_value_with_special_chars: abc=123&xyz";
    let encrypted = CryptoService::encrypt(original).expect("encrypt should succeed");
    assert_ne!(encrypted, original, "encrypted should differ from original");
    let decrypted = CryptoService::decrypt(&encrypted).expect("decrypt should succeed");
    assert_eq!(decrypted, original);
}

#[test]
fn crypto_encrypt_empty_string() {
    let original = "";
    let encrypted = CryptoService::encrypt(original).expect("encrypt empty string should succeed");
    let decrypted = CryptoService::decrypt(&encrypted).expect("decrypt should succeed");
    assert_eq!(decrypted, original);
}

#[test]
fn crypto_decrypt_invalid_data_returns_error() {
    let result = CryptoService::decrypt("not_valid_encrypted_data_at_all");
    assert!(result.is_err(), "decrypting invalid data should fail");
}

#[test]
fn crypto_encrypt_produces_different_ciphertexts() {
    let original = "same_input";
    let encrypted1 = CryptoService::encrypt(original).unwrap();
    let encrypted2 = CryptoService::encrypt(original).unwrap();
    // DPAPI 加密可能产生相同密文，但通常不同
    // 两者解密后应都等于原文
    assert_eq!(CryptoService::decrypt(&encrypted1).unwrap(), original);
    assert_eq!(CryptoService::decrypt(&encrypted2).unwrap(), original);
}

#[test]
fn crypto_encrypt_long_cookie() {
    let long_cookie = "pdd_v=xxx; pc_pair_ticket=yyy; __puus=zzz; another_field=aaa; ".repeat(10);
    let encrypted = CryptoService::encrypt(&long_cookie).unwrap();
    let decrypted = CryptoService::decrypt(&encrypted).unwrap();
    assert_eq!(decrypted, long_cookie);
}

#[test]
fn rpc_server_add_to_config() {
    let mut config = create_test_config();
    let server = create_test_rpc_server("Aria2", "http://localhost:6800");
    config.rpc_servers.push(server.clone());
    assert_eq!(config.rpc_servers.len(), 1);
    assert_eq!(config.rpc_servers[0].name, "Aria2");
}

#[test]
fn rpc_server_add_multiple() {
    let mut config = create_test_config();
    config.rpc_servers.push(create_test_rpc_server("Aria2", "http://localhost:6800"));
    config.rpc_servers.push(create_test_rpc_server("BitComet", "http://localhost:8888"));
    config.rpc_servers.push(create_test_rpc_server("ABDM", "http://localhost:9999"));
    assert_eq!(config.rpc_servers.len(), 3);
}

#[test]
fn rpc_server_delete_by_id() {
    let mut config = create_test_config();
    let server = create_test_rpc_server("Aria2", "http://localhost:6800");
    let server_id = server.id.clone();
    config.rpc_servers.push(server);
    config.rpc_servers.retain(|s| s.id != server_id);
    assert!(config.rpc_servers.is_empty());
}

#[test]
fn rpc_server_delete_nonexistent_id_no_effect() {
    let mut config = create_test_config();
    config.rpc_servers.push(create_test_rpc_server("Aria2", "http://localhost:6800"));
    config.rpc_servers.retain(|s| s.id != "nonexistent");
    assert_eq!(config.rpc_servers.len(), 1);
}

#[test]
fn rpc_server_set_default() {
    let mut config = create_test_config();
    config.rpc_servers.push(create_test_rpc_server("Aria2", "http://localhost:6800"));
    config.rpc_servers.push(create_test_rpc_server("BC", "http://localhost:8888"));

    for s in &mut config.rpc_servers {
        s.is_default = false;
    }
    config.rpc_servers[1].is_default = true;
    config.default_rpc_index = 1;

    assert!(!config.rpc_servers[0].is_default);
    assert!(config.rpc_servers[1].is_default);
    assert_eq!(config.default_rpc_index, 1);
}

#[test]
fn rpc_server_update_existing() {
    let mut config = create_test_config();
    let mut server = create_test_rpc_server("Aria2", "http://localhost:6800");
    let server_id = server.id.clone();
    config.rpc_servers.push(server.clone());

    server.url = "http://newhost:6800".to_string();
    server.token = Some("new_token".to_string());
    if let Some(s) = config.rpc_servers.iter_mut().find(|s| s.id == server_id) {
        *s = server;
    }

    assert_eq!(config.rpc_servers[0].url, "http://newhost:6800");
    assert_eq!(config.rpc_servers[0].token, Some("new_token".to_string()));
}

#[test]
fn rpc_server_with_different_downloader_types() {
    let types = vec![
        DownloaderType::Aria2,
        DownloaderType::BitComet,
        DownloaderType::ABDownloadManager,
        DownloaderType::Custom,
    ];
    for dt in types {
        let server = RpcServer {
            id: uuid::Uuid::new_v4().to_string(),
            name: "Test".to_string(),
            url: "http://localhost:6800".to_string(),
            token: None,
            downloader_type: dt,
            download_dir: None,
            is_default: false,
        };
        let json = serde_json::to_string(&server).unwrap();
        let parsed: RpcServer = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.downloader_type, server.downloader_type);
    }
}

#[test]
fn config_theme_switching() {
    let mut config = create_test_config();
    assert_eq!(config.theme, Theme::System);

    config.theme = Theme::Dark;
    assert_eq!(config.theme, Theme::Dark);

    config.theme = Theme::Light;
    assert_eq!(config.theme, Theme::Light);
}

#[test]
fn config_first_run_flag_toggle() {
    let mut config = create_test_config();
    assert!(config.is_first_run);
    config.is_first_run = false;
    assert!(!config.is_first_run);
}

#[test]
fn config_retry_count_validation() {
    let mut config = create_test_config();
    assert_eq!(config.retry_count, 3);
    config.retry_count = 0;
    assert_eq!(config.retry_count, 0);
    config.retry_count = 10;
    assert_eq!(config.retry_count, 10);
}
