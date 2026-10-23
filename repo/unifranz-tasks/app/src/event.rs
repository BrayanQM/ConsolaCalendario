use crossterm::event::{self, Event as CEvent};
use std::time::Duration;
use tokio::sync::mpsc;

#[derive(Clone, Debug)]
pub enum Event {
    Tick,
    Key(crossterm::event::KeyEvent),
    Mouse(crossterm::event::MouseEvent),
    Resize(u16, u16),
}

pub struct EventHandler {
    rx: mpsc::UnboundedReceiver<Event>,
    _tx: mpsc::UnboundedSender<Event>,
}

impl EventHandler {
    pub fn new(tick_rate: Duration) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        let tx_clone = tx.clone();
        tokio::spawn(async move {
            let mut tick = tokio::time::interval(tick_rate);
            loop {
                tokio::select! {
                    _ = tick.tick() => { let _ = tx_clone.send(Event::Tick); }
                    res = tokio::task::spawn_blocking(|| event::read()) => {
                        if let Ok(Ok(c)) = res {
                            match c {
                                CEvent::Key(k) => { let _ = tx_clone.send(Event::Key(k)); }
                                CEvent::Mouse(m) => { let _ = tx_clone.send(Event::Mouse(m)); }
                                CEvent::Resize(w,h) => { let _ = tx_clone.send(Event::Resize(w,h)); }
                                _ => {}
                            }
                        }
                    }
                }
            }
        });
        Self { rx, _tx: tx }
    }

    pub async fn next(&mut self) -> anyhow::Result<Event> {
        self.rx.recv().await.ok_or_else(|| anyhow::anyhow!("channel closed"))
    }
}
