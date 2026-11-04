use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{
        Color::{self, Rgb},
        Modifier, Style, Stylize,
        palette::tailwind::SLATE,
    },
    text::Line,
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, ListState, Paragraph},
};

use crate::app::{App, ConnectionState, Overlay, Screen};

#[derive(Clone, Copy)]
pub struct Theme {
    pub border: Style,
    pub text: Style,
    pub muted_text: Style,
}

impl Theme {
    pub fn normal() -> Self {
        Self {
            border: Style::default().fg(Color::Yellow).bg(Color::Black),
            text: Style::default().fg(Color::Yellow),
            muted_text: Style::default().fg(Color::White),
        }
    }

    pub fn dimmed() -> Self {
        Self {
            border: Style::default().fg(Rgb(64, 64, 64)).bg(Color::Black),
            text: Style::default().fg(Rgb(64, 64, 64)),
            muted_text: Style::default().fg(Rgb(64, 64, 64)),
        }
    }
}

pub fn render(app: &App, frame: &mut Frame) {
    let area = frame.area();

    let normal_theme = Theme::normal();

    let is_dimmed = app.overlay != Overlay::None;
    let theme = if is_dimmed {
        Theme::dimmed()
    } else {
        Theme::normal()
    };

    match app.screen {
        Screen::Main => {
            render_main(area, app, frame, theme);

            match app.overlay {
                Overlay::None => {}
                Overlay::PasswordInput => todo!(),
                Overlay::ConfirmDisconnect => todo!(),
                Overlay::Menu { mut state } => render_menu(area, frame, &mut state, normal_theme),
                Overlay::About => render_about(area, frame, normal_theme),
            }
        }
        Screen::KnownNetworks => todo!(),
        Screen::NetworkDetails => todo!(),
        Screen::Interfaces => todo!(),
    }
}

fn render_main(area: Rect, app: &App, frame: &mut Frame, theme: Theme) {
    let outer = Block::default()
        .title(" Crabwire ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(theme.border);

    let status_block = outer.inner(area);
    frame.render_widget(outer, area);

    let [head, networks, status, help] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(3),
        Constraint::Length(3),
        Constraint::Length(1),
    ])
    .areas(status_block);

    frame.render_widget(
        Paragraph::new("Activate a connection")
            .alignment(Alignment::Center)
            .style(theme.text),
        head,
    );

    let list_network = app
        .networks
        .iter()
        .map(|network| ListItem::new(network.ssid.as_str()))
        .collect::<Vec<_>>();
    let inner_network = Block::default()
        .borders(Borders::TOP | Borders::BOTTOM)
        .style(theme.border);

    frame.render_widget(List::new(list_network), networks);

    let text = match &app.connection_state {
        ConnectionState::Connected(info) => {
            format!("{}  ○ {}", app.connection_state, info.interface)
        }
        _ => app.connection_state.to_string(),
    };

    frame.render_widget(
        Paragraph::new(text).block(inner_network).style(theme.text),
        status,
    );

    frame.render_widget(
        Paragraph::new("↑↓ Select   Enter Activate   Esc Menu   q Quit").style(theme.text),
        help,
    );
}

fn render_menu(area: Rect, frame: &mut Frame, state: &mut ListState, theme: Theme) {
    let popup_area = area.centered(Constraint::Length(30), Constraint::Length(9));

    let popup_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(theme.border);

    let menu_area = popup_block.inner(popup_area);

    frame.render_widget(Clear, popup_area);
    frame.render_widget(popup_block, popup_area);

    let [head, area_for_items] =
        Layout::vertical([Constraint::Length(2), Constraint::Min(5)]).areas(menu_area);

    frame.render_widget(
        Paragraph::new("Please select an option")
            .style(theme.text)
            .centered(),
        head,
    );

    let items = [
        ListItem::new(Line::from("Known Networks").centered()),
        ListItem::new(Line::from("Interfaces").centered()),
        ListItem::new(Line::from("Keybinds").centered()),
        ListItem::new(Line::from("")),
        ListItem::new(Line::from("About").centered()),
    ];

    let list = List::new(items)
        .highlight_style(Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD))
        .style(theme.muted_text);

    frame.render_stateful_widget(list, area_for_items, state);
}

fn render_about(area: Rect, frame: &mut Frame, theme: Theme) {
    let popup_area = area.centered(Constraint::Length(50), Constraint::Length(9));

    frame.render_widget(Clear, popup_area);

    let block = Block::default()
        .title(" About ")
        .title_bottom(Line::from(" Esc Close ").alignment(Alignment::Right))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(theme.border);

    let text = vec![
        Line::from("Crabwire").style(Style::default().bold()),
        Line::from(""),
        Line::from("A lightweight TUI network manager for Linux"),
        Line::from(""),
        Line::from(format!("Version {}", env!("CARGO_PKG_VERSION"))),
        Line::from(""),
        Line::from("Made by Nizarjh"),
    ];

    let paragraph = Paragraph::new(text).style(theme.muted_text).centered();

    let inner_area = block.inner(popup_area);
    frame.render_widget(block, popup_area);
    frame.render_widget(paragraph, inner_area);
}
