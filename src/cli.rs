//! Module `cli` contains the main command interface functionality.

use crate::config::{WoafConfig, get_config, print_config};
use clap::Parser;

/// CLIArgs represents the command line arguments for the application.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CLIArgs {
    /// Enable debug mode
    #[arg(short = 'd', long = "debug", help = "Enable debug mode")]
    pub debug: bool,
    /// Flag to enable upload mode (default is download mode)
    #[arg(
        short = 'u',
        long = "upload",
        help = "Flag to enable upload mode (default is download mode)"
    )]
    pub upload: bool,
    /// Path to the config file (disables all other config sources)
    #[arg(
        short = 'c',
        long = "config-file",
        help = "Path to the config file (disables all other config files, including global)"
    )]
    pub config_file: Option<String>,
    /// Enable in-memory caching
    #[arg(long = "memory-cache", help = "Enable in-memory caching")]
    pub memory_cache: bool,
    /// Set the archive method (bz2, gzip, tar, zip, zstd)
    #[arg(
        long = "archive-method",
        help = "Set the archive method (bz2, gzip, tar, zip, zstd)"
    )]
    pub archive_method: Option<String>,
    /// Set the maximum upload size in bytes
    #[arg(
        short = 's',
        long = "maximum-upload-size",
        help = "Set the maximum upload size in bytes"
    )]
    pub maximum_upload_size: Option<u64>,
    /// Set the maximum download count
    #[arg(long = "download-limit", help = "Set the maximum download count")]
    pub download_limit: Option<u64>,
    /// Set the maximum upload count
    #[arg(long = "upload-limit", help = "Set the maximum upload count")]
    pub upload_limit: Option<u64>,
    /// Set the listen IP address
    #[arg(short = 'i', long = "listen-ip", help = "Set the listen IP address")]
    pub listen_ip: Option<String>,
    /// Set the listen port
    #[arg(short = 'p', long = "listen-port", help = "Set the listen port")]
    pub listen_port: Option<u16>,
    /// Set the hostnames (comma-separated)
    #[arg(
        short = 'H',
        long = "hostnames",
        help = "Set the hostnames (comma-separated)"
    )]
    pub hostnames: Option<String>,
    /// Enable HTTPS
    #[arg(long = "https", help = "Enable HTTPS")]
    pub https: Option<bool>,
    /// Set the key passphrase
    #[arg(long = "passphrase", help = "Set the key passphrase")]
    pub key_passphrase: Option<String>,
    /// Set the path to the key file
    #[arg(long = "key", help = "Set the path to the key file")]
    pub key_path: Option<String>,
    /// Set the path to the certificate file
    #[arg(long = "cert", help = "Set the path to the certificate file")]
    pub cert_path: Option<String>,
    /// Set the target directory or file for the application to operate on
    /// This is a positional argument and is required.
    #[arg(
        help = "Set the target directory or file to offer for download. For upload mode, this must be a directory and already exist."
    )]
    pub target: String,
}

/// Run the command line interface.
///
/// This is the true main function of the application as called from main.rs. This is mainly to allow main.rs to only contain the
/// crate rustdoc and module definitions.
pub fn run() {
    let args = CLIArgs::parse();
    let config: WoafConfig = get_config(&args).unwrap();
    print_config(&config);
    println!("Target: {}", args.target);
    println!("Mode: {}", if args.upload { "Upload" } else { "Download" });
}
