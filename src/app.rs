use std::{fmt, net::IpAddr};

use ratatui::widgets::ListState;

use crate::event::Event;

/// Application.
#[derive(Debug, Default)]
pub struct App {
    // should the application exit?
    pub should_quit: bool,
    // selected network in Main menu
    pub selected_network: usize,
    // what in focus statusbar or what
    pub focus: Focus,
    // screens
    pub screen: Screen,
    // overlays
    pub overlay: Overlay,
    // Connection state
    pub connection_state: ConnectionState,
    // Vector of errors
    pub notifications: Vec<Event>,
    // vector of wifi networks
    pub networks: Vec<Network>,
    // saved networks
    pub saved_networks: Vec<Network>,
}

#[derive(Debug, Default)]
pub enum Screen {
    #[default]
    Main,
    KnownNetworks,
    NetworkDetails,
    Interfaces,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum Overlay {
    #[default]
    None,
    PasswordInput,
    ConfirmDisconnect,
    Menu {
        state: ListState,
    },
    About,
}

#[derive(Debug, Default)]
pub enum ConnectionState {
    #[default]
    Disconnected,
    Connecting,
    Connected(ConnectionInfo),
}

#[derive(Debug, Default)]
pub enum Focus {
    #[default]
    NetworkList,
    StatusBar,
}

#[derive(Debug, Default)]
pub struct ConnectionInfo {
    pub interface: String,
    pub network: String,
    pub ip: Option<IpAddr>,
}

#[derive(Debug, Default)]
pub struct Network {
    pub ssid: String,
    pub signal_dbm: i32,
    pub known: bool,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }
    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}

impl fmt::Display for ConnectionState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConnectionState::Connected(_) => write!(f, "Connected"),
            ConnectionState::Disconnected => write!(f, "Disconnected"),
            ConnectionState::Connecting => write!(f, "Connecting"),
        }
    }
}
