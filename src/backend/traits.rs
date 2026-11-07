use crate::models::{
    connect::ConnectRequest,
    interface::{Interface, InterfaceCapabilities},
    network::{AccessPoint, SavedNetwork},
};
use anyhow::Result;

//TODO: impl for wpa_supplicant and iwd
pub trait WifiBackend {
    async fn interfaces(&self) -> Result<Vec<Interface>>;
    async fn capabilities(&self, interface: &str) -> Result<InterfaceCapabilities>;
}
pub trait StationBackend: WifiBackend {
    async fn scan(&self, interface: &str) -> Result<Vec<AccessPoint>>;
    async fn connect(&self, request: ConnectRequest) -> Result<()>;
    async fn disconnect(&self, interface: &str) -> Result<()>;
    async fn saved_networks(&self, interface: &str) -> Result<Vec<SavedNetwork>>;
}
