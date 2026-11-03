use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    widgets::ListState,
};

use crate::app::{App, Overlay, Screen};

pub async fn handle_key_event(app: &mut App, key_event: KeyEvent) {
    let len_of_networks = app.networks.len();
    match app.screen {
        Screen::Main => handle_key_event_for_main(key_event.code, app, len_of_networks),
        Screen::KnownNetworks => todo!(),
        Screen::NetworkDetails => todo!(),
        Screen::Interfaces => todo!(),
    }
}

fn handle_key_event_for_main(keycode: KeyCode, app: &mut App, len_of_networks: usize) {
    match &mut app.overlay {
        Overlay::None => match keycode {
            KeyCode::Char('q') => app.quit(),
            KeyCode::Esc => {
                app.overlay = Overlay::Menu {
                    state: ListState::default().with_selected(Some(0)),
                };
            }
            KeyCode::Up => {}
            KeyCode::Down => {}
            _ => {}
        },
        Overlay::PasswordInput => todo!(),
        Overlay::ConfirmDisconnect => todo!(),
        Overlay::Menu { state } => match keycode {
            KeyCode::Char('q') => app.quit(),

            KeyCode::Esc => {
                app.overlay = Overlay::None;
            }

            KeyCode::Up => match state.selected() {
                Some(0) | None => state.select(Some(4)),
                Some(4) => state.select(Some(2)),
                Some(i) => state.select(Some(i - 1)),
            },

            KeyCode::Down => match state.selected() {
                Some(4) => state.select(Some(0)),
                Some(2) => state.select(Some(4)),
                Some(i) => state.select(Some(i + 1)),
                None => state.select(Some(0)),
            },

            _ => {}
        },
    }
}
