use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpConfig {
    pub read_timeout: u64,
    pub pool_idle_timeout: u64,
    pub pool_max_idle_per_host: usize,
    pub max_redirects: usize,
    pub ssl_cert_file: Option<String>,
    #[serde(default)]
    pub danger_accept_invalid_certs: bool,
}

impl Default for HttpConfig {
    fn default() -> Self {
        Self {
            read_timeout: 60,
            pool_idle_timeout: 90,
            pool_max_idle_per_host: 5,
            max_redirects: 10,
            ssl_cert_file: None,
            danger_accept_invalid_certs: false,
        }
    }
}
