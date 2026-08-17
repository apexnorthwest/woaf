//! Module file-offer contains the server components used when offering files for download.
//!
//! This module is based on hyper.

use base64::Engine;
use hyper::Request;
use hyper::Response;
use hyper::body::{Bytes, Incoming};
use hyper::service::service_fn;
use rand::prelude::*;
use rustls::ServerConfig;
use rustls::pki_types::pem::PemObject;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use std::sync::Arc;
use tokio_rustls::TlsAcceptor;

#[allow(dead_code)]
pub fn offer_file(
    file_handle: std::fs::File,
    file_name: String,
    port: u16,
    hostnames: Vec<String>,
    https: bool,
    key_path: Option<String>,
    cert_path: Option<String>,
    key_passphrase: Option<String>,
) {
    let session_secret = generate_session_secret();
    if https {
        // Setup tls parameters
        let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
        let certs = CertificateDer::pem_file_iter(cert_path.unwrap())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| error(format!("could not read certificate file: {e}")))
            .unwrap();
        let key = PrivateKeyDer::from_pem_file(key_path.unwrap())
            .map_err(|e| error(format!("could not read private key file: {e}")))
            .unwrap();
        let listener = std::net::TcpListener::bind(format!("0.0.0.0:{}", port))
            .map_err(|e| error(format!("could not bind to port {}: {e}", port)))
            .unwrap();
        let mut server_config = ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(certs, key)
            .map_err(|e| error(e.to_string()))?;
        server_config.alpn_protocols =
            vec![b"h2".to_vec(), b"http/1.1".to_vec(), b"http/1.0".to_vec()];
        let tls_acceptor = TlsAcceptor::from(Arc::new(server_config));
        let service = service_fn(filesend(file_handle));
    } else {
        let listener = std::net::TcpListener::bind(format!("0.0.0.0:{}", port))
            .map_err(|e| error(format!("could not bind to port {}: {e}", port)))
            .unwrap();
    }
}

async fn filesend(req: Request<Incoming>) -> Result<Response<Bytes>, hyper::Error> {}

fn generate_session_secret() -> String {
    let mut rng = rand::rng();
    let mut random_bytes: [u8; 36] = [0; 36];
    rng.fill(&mut random_bytes);
    let e = base64::engine::general_purpose::URL_SAFE_NO_PAD;
    e.encode(&random_bytes)
}

fn error(msg: String) -> ! {
    eprintln!("{}", msg);
    std::process::exit(1);
}
