use anyhow::Result;
use crossterm::event::{Event as CrosstermEvent, KeyEvent, MouseEvent};
use futures::{FutureExt, StreamExt};
use std::time::Duration;
use tokio::sync::mpsc;

/// Terminal events.
#[derive(Clone, Copy, Debug)]
pub enum Event {
    /// Terminal tick.
    Tick,
    /// Key press.
    Key(KeyEvent),
    /// Mouse click/scroll.
    Mouse(MouseEvent),
    /// Terminal resize.
    Resize(u16, u16),
}

#[derive(Debug)]
pub struct EventHandler {
    pub sender: mpsc::UnboundedSender<Event>,
    pub receiver: mpsc::UnboundedReceiver<Event>,
    #[allow(dead_code)]
    handler: tokio::task::JoinHandle<()>,
}

impl EventHandler {
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = mpsc::unbounded_channel();
        let sender_cloned = sender.clone();
        let handler = tokio::spawn(async move {
            let mut reader = crossterm::event::EventStream::new();
            let mut interval = tokio::time::interval(tick_rate);
            loop {
                let tick_delay = interval.tick();
                let crossterm_event = reader.next().fuse();

                tokio::select! {
                    () = sender_cloned.closed() => {
                        break;
                    }

                    _ = tick_delay => {
                        if sender_cloned.send(Event::Tick).is_err() {
                            break;
                        }
                    }

                    Some(Ok(evt)) = crossterm_event => {
                        match evt {
                            CrosstermEvent::Key(key)
                                if key.kind == crossterm::event::KeyEventKind::Press => {
                                    if sender_cloned.send(Event::Key(key)).is_err() {
                                        break;
                                    }
                                }
                            CrosstermEvent::Resize(x, y) => {
                                if sender_cloned.send(Event::Resize(x, y)).is_err() {
                                    break;
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        });
        Self {
            sender,
            receiver,
            handler,
        }
    }
    pub async fn next(&mut self) -> Result<Event> {
        self.receiver
            .recv()
            .await
            .ok_or(std::io::Error::other("This is an IO error").into())
    }
}
