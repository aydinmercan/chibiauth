use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    server: ServerConfig,
}

#[derive(Deserialize)]
pub struct ServerConfig {
    host: String,
    port: u32,

    #[serde(rename = "base-url")]
    base_url: String,

    tls: TlsConfig,
}

#[derive(Deserialize)]
pub struct TlsConfig {
    offload: bool,

    certificate: PathBuf,

    #[serde(rename = "private-key")]
    private_key: PathBuf,
}
