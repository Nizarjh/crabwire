use crate::models::network::AuthConfig;

pub struct ConnectRequest {
    pub interface: String,
    pub ssid: String,
    pub bssid: Option<String>,
    pub auth: AuthConfig,
}
