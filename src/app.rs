use ratatui::widgets::ListState;

use crate::{
    event::Event,
    models::{
        interface::InterfaceState,
        network::{AccessPoint, SavedNetwork},
    },
};

/// Application.
#[derive(Default)]
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
    pub connection_state: InterfaceState,
    // Vector of errors
    pub notifications: Vec<Event>,
    // vector of wifi networks
    pub networks: Vec<AccessPoint>,
    // saved networks
    pub saved_networks: Vec<SavedNetwork>,
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
pub enum Focus {
    #[default]
    NetworkList,
    StatusBar,
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
