use std::{fmt, net::IpAddr};

pub struct Interface {
    pub name: String,
    pub mac_address: Option<String>,
    pub state: InterfaceState,
    pub is_active: bool,
}

pub struct InterfaceCapabilities {
    pub station: bool,
    pub access_point: bool,
}

#[derive(Debug, Default)]
pub struct ConnectionInfo {
    pub network: String,
    pub ip: Option<IpAddr>,
}

#[derive(Debug, Default)]
pub enum InterfaceState {
    #[default]
    Disconnected,
    Connecting,
    Connected(ConnectionInfo),
}


impl fmt::Display for InterfaceState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InterfaceState::Connected(_) => write!(f, "Connected"),
            InterfaceState::Disconnected => write!(f, "Disconnected"),
            InterfaceState::Connecting => write!(f, "Connecting"),
        }
    }
}
