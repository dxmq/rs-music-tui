use crate::event::key::Key;
use crossterm::event;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub enum Event<I> {
    Input(I),
    Tick,
}
pub struct Events {
    rx: mpsc::Receiver<Event<Key>>,
    _tx: mpsc::Sender<Event<Key>>,
}

pub struct EventConfig {
    pub exit_key: Key,
    pub tick_rate: Duration,
}

impl Default for EventConfig {
    fn default() -> Self {
        Self {
            exit_key: Key::Ctrl('c'),
            tick_rate: Duration::from_millis(250),
        }
    }
}

impl Events {
    pub fn new(tick_rate: u64) -> Self {
        Events::with_config(EventConfig {
            tick_rate: Duration::from_millis(tick_rate),
            ..EventConfig::default()
        })
    }

    fn with_config(config: EventConfig) -> Events {
        let (tx, rx) = mpsc::channel();
        let event_tx = tx.clone();

        thread::spawn(move || loop {
            if event::poll(config.tick_rate).unwrap() {
                if let event::Event::Key(key_event) = event::read().unwrap() {
                    let key = Key::from(key_event);
                    event_tx.send(Event::Input(key));
                }
            }

            event_tx.send(Event::Tick).unwrap()
        });
        Events { rx, _tx: tx }
    }
}
