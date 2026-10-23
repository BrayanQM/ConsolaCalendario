use crossterm::event::{self, Event as CEvent, KeyEvent};
use std::time::Duration;

#[derive(Clone, Debug)]
pub enum Event {
    Tick,
    Key(KeyEvent),
    Mouse(crossterm::event::MouseEvent),
    Resize(u16, u16),
}

pub struct EventHandler {
    tick_rate: Duration,
}

impl EventHandler {
    pub fn new(tick_rate: Duration) -> Self {
        Self { tick_rate }
    }

    pub fn next_blocking(&self) -> std::io::Result<Option<Event>> {
        if event::poll(self.tick_rate)? {
            match event::read()? {
                CEvent::Key(k) => Ok(Some(Event::Key(k))),
                CEvent::Mouse(m) => Ok(Some(Event::Mouse(m))),
                CEvent::Resize(w,h) => Ok(Some(Event::Resize(w,h))),
                _ => Ok(Some(Event::Tick)),
            }
        } else {
            Ok(Some(Event::Tick))
        }
    }
}
