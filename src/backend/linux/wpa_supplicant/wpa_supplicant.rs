use crate::{
    backend::traits::{StationBackend, WifiBackend},
    models::{
        connect::ConnectRequest,
        interface::{Interface, InterfaceCapabilities},
        network::{AccessPoint, SavedNetwork},
    },
};

pub struct WpaSupplicant {}

impl WifiBackend for WpaSupplicant {
    async fn interfaces(&self) -> anyhow::Result<Vec<Interface>> {
        todo!()
    }

    async fn capabilities(&self, interface: &str) -> anyhow::Result<InterfaceCapabilities> {
        todo!()
    }
}

impl StationBackend for WpaSupplicant {
    async fn scan(&self, interface: &str) -> anyhow::Result<Vec<AccessPoint>> {
        todo!()
    }

    async fn connect(&self, request: ConnectRequest) -> anyhow::Result<()> {
        todo!()
    }

    async fn disconnect(&self, interface: &str) -> anyhow::Result<()> {
        todo!()
    }

    async fn saved_networks(&self, interface: &str) -> anyhow::Result<Vec<SavedNetwork>> {
        todo!()
    }
}
