//! Module `config` handles parsing config files, cli options, and setting defaults

use crate::cli::{CLIArgs};
use serde::Deserialize;
use std::fs::exists;
use std::fmt;

#[allow(dead_code)]
/// ArchiveMethod represents the supported archive methods for the application.
pub enum ArchiveMethod {
    Bz2,
    Gzip,
    Tar,
    Zip,
    Zstd,
}

/// Implement Debug for ArchiveMethod to display the archive method as a string.
impl fmt::Debug for ArchiveMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArchiveMethod::Bz2 => write!(f, "bz2"),
            ArchiveMethod::Gzip => write!(f, "gzip"),
            ArchiveMethod::Tar => write!(f, "tar"),
            ArchiveMethod::Zip => write!(f, "zip"),
            ArchiveMethod::Zstd => write!(f, "zstd"),
        }
    }
}

/// WoafConfig represents the configuration of the application.
/// This is the public interface for the configuration and is used throughout the application.
pub struct WoafConfig {
    memory_cache: bool,
    archive_method: ArchiveMethod,
    maximum_upload_size: u64,
    download_limit: u64,
    upload_limit: u64,
    listen_ip: String,
    listen_port: u16,
    hostnames: Option<Vec<String>>,
    https_enabled: bool,
    https_key_passphrase: Option<String>,
    https_key_path: Option<String>,
    https_cert_path: Option<String>,
}

/// WoafTOMLConfig represents the top-level configuration of the TOML config file.
/// Used to unpack the config file using serde.
#[derive(Deserialize)]
struct WoafTOMLConfig {
    memory_cache: Option<bool>,
    archive_method: Option<String>,
    maximum_upload_size: Option<u64>,
    download_limit: Option<u64>,
    upload_limit: Option<u64>,
    listen_ip: Option<String>,
    listen_port: Option<u16>,
    hostnames: Option<Vec<String>>,
    https: Option<WoafHTTPSConfig>,
}

/// WoafHTTPSConfig represents the \[https\] configuration section of the TOML config file.
/// Used by serde to unpack the config file.
#[derive(Deserialize)]
struct WoafHTTPSConfig {
    enabled: Option<bool>,
    key_passphrase: Option<String>,
    key_path: Option<String>,
    cert_path: Option<String>,
}

/// Parse the configuration file at the given path and return a `Table` representation of it.
pub fn get_config(args: &CLIArgs) -> Result<WoafConfig, String> {
    let mut merged: WoafConfig = get_defaults();
    if let Some(ref config_file_path) = args.config_file {
        if exists(config_file_path).unwrap() {
            let config_str = std::fs::read_to_string(config_file_path).map_err(|e| format!("Failed to read config file: {}", e))?;
            let config: WoafTOMLConfig = toml::from_str(&config_str).map_err(|e| format!("Failed to parse config file: {}", e))?;
            merged = merge_configs(merged, config).map_err(|e| format!("Failed to merge config file: {}", e))?;
        } else {
            return Err(format!("Config file not found at path: {}", config_file_path));
        }
    } else {
        if exists("/etc/woaf/woaf.toml").unwrap() {
            let system_config_str = std::fs::read_to_string("/etc/woaf/woaf.toml").map_err(|e| format!("Failed to read config file: {}", e))?;
            let system_config: WoafTOMLConfig = toml::from_str(&system_config_str).map_err(|e| format!("Failed to parse config file: {}", e))?;
            merged = merge_configs(merged, system_config).map_err(|e| format!("Failed to merge config file: {}", e))?;
        }
        if exists(std::env::home_dir().unwrap().join(".config/woaf.toml")).unwrap() {
            let user_config_str = std::fs::read_to_string(std::env::home_dir().unwrap().join(".config/woaf.toml")).map_err(|e| format!("Failed to read config file: {}", e))?;
            let user_config: WoafTOMLConfig = toml::from_str(&user_config_str).map_err(|e| format!("Failed to parse config file: {}", e))?;
            merged = merge_configs(merged, user_config).map_err(|e| format!("Failed to merge config file: {}", e))?;
        }
    }
    let cli_config: WoafTOMLConfig = convert_cliargs_to_tomlconfig(args);
    
    merged = merge_configs(merged, cli_config).map_err(|e| format!("Failed to merge config file: {}", e))?;
    Ok(merged)
}

/// Get the default configuration values for the application.
fn get_defaults() -> WoafConfig {
    WoafConfig {
        memory_cache: true,
        archive_method: ArchiveMethod::Zip,
        maximum_upload_size: 8589934592, // 8 GiB
        download_limit: 1,
        upload_limit: 1,
        listen_ip: "0.0.0.0".to_string(),
        listen_port: 8080,
        hostnames: None,
        https_enabled: false,
        https_key_passphrase: None,
        https_key_path: None,
        https_cert_path: None,
    }
}

/// Convert CLIArgs to WoafTOMLConfig for merging with other configs.
fn convert_cliargs_to_tomlconfig(args: &CLIArgs) -> WoafTOMLConfig {
    WoafTOMLConfig {
        memory_cache: Some(args.memory_cache),
        archive_method: args.archive_method.clone(),
        maximum_upload_size: args.maximum_upload_size,
        download_limit: args.download_limit,
        upload_limit: args.upload_limit,
        listen_ip: args.listen_ip.clone(),
        listen_port: args.listen_port,
        hostnames: args.hostnames.clone().map(|h| h.split(',').map(|s| s.trim().to_string()).collect()),
        https: args.https.map(|enabled| WoafHTTPSConfig {
            enabled: Some(enabled),
            key_passphrase: args.key_passphrase.clone(),
            key_path: args.key_path.clone(),
            cert_path: args.cert_path.clone(),
        }),
    }
}

/// Merges a given configuration with the imported version of a toml config file, with the toml having priority.
fn merge_configs(base: WoafConfig, override_config: WoafTOMLConfig) -> Result<WoafConfig, String> {
    let mut merged = base;
    if let Some(memory_cache) = override_config.memory_cache {
        merged.memory_cache = memory_cache;
    }
    if let Some(archive_method_str) = override_config.archive_method {
        merged.archive_method = match archive_method_str.as_str() {
            "bz2" => ArchiveMethod::Bz2,
            "gzip" => ArchiveMethod::Gzip,
            "tar" => ArchiveMethod::Tar,
            "zip" => ArchiveMethod::Zip,
            "zstd" => ArchiveMethod::Zstd,
            _ => return Err(format!("Invalid archive method: {}", archive_method_str.as_str())),
        };
    }
    if let Some(maximum_upload_size) = override_config.maximum_upload_size {
        merged.maximum_upload_size = maximum_upload_size;
    }
    if let Some(download_limit) = override_config.download_limit {
        merged.download_limit = download_limit;
    }
    if let Some(upload_limit) = override_config.upload_limit {
        merged.upload_limit = upload_limit;
    }
    if let Some(listen_ip) = override_config.listen_ip {
        merged.listen_ip = listen_ip;
    }
    if let Some(listen_port) = override_config.listen_port {
        merged.listen_port = listen_port;
    }
    if let Some(hostnames) = override_config.hostnames {
        merged.hostnames = Some(hostnames);
    }
    if let Some(https_config) = override_config.https {
        if let Some(enabled) = https_config.enabled {
            merged.https_enabled = enabled;
        }
        if let Some(key_passphrase) = https_config.key_passphrase {
            merged.https_key_passphrase = Some(key_passphrase);
        }
        if let Some(key_path) = https_config.key_path {
            merged.https_key_path = Some(key_path);
        }
        if let Some(cert_path) = https_config.cert_path {
            merged.https_cert_path = Some(cert_path);
        }
    }
    Ok(merged)
}

/// Print the configuration to stdout for debugging purposes.
pub fn print_config(config: &WoafConfig) {
    println!("Configuration:");
    println!("  Memory Cache: {}", config.memory_cache);
    println!("  Archive Method: {:?}", config.archive_method);
    println!("  Maximum Upload Size: {}", config.maximum_upload_size);
    println!("  Download Limit: {}", config.download_limit);
    println!("  Upload Limit: {}", config.upload_limit);
    println!("  Listen IP: {}", config.listen_ip);
    println!("  Listen Port: {}", config.listen_port);
    if let Some(hostnames) = &config.hostnames {
        println!("  Hostnames: {:?}", hostnames);
    } else {
        println!("  Hostnames: None");
    }
    println!("  HTTPS Enabled: {}", config.https_enabled);
    if let Some(passphrase) = &config.https_key_passphrase {
        println!("  HTTPS Key Passphrase: {}", passphrase);
    } else {
        println!("  HTTPS Key Passphrase: None");
    }
    if let Some(key_path) = &config.https_key_path {
        println!("  HTTPS Key Path: {}", key_path);
    } else {
        println!("  HTTPS Key Path: None");
    }
    if let Some(cert_path) = &config.https_cert_path {
        println!("  HTTPS Cert Path: {}", cert_path);
    } else {
        println!("  HTTPS Cert Path: None");
    }
}