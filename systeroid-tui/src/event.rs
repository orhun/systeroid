use std::io;
use std::sync::mpsc::{self, Receiver, RecvError, Sender};
use std::thread;
use std::time::Duration;
use termion::event::Key;
use termion::input::TermRead;

/// Representation of terminal events.
#[derive(Clone, Copy, Debug)]
pub enum Event {
    /// A key press event.
    KeyPress(Key),
    /// A terminal refresh event.
    Tick,
}

/// [`Event`] handler.
#[derive(Debug)]
pub struct EventHandler {
    /// Sender channel for the events.
    #[allow(dead_code)]
    sender: Sender<Event>,
    /// Receiver channel for the events.
    receiver: Receiver<Event>,
}

impl EventHandler {
    /// Constructs a new event handler with the given refresh rate.
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = mpsc::channel();
        {
            let sender = sender.clone();
            thread::spawn(move || loop {
                io::stdin()
                    .keys()
                    .flatten()
                    .try_for_each(|key| sender.send(Event::KeyPress(key)))
                    .expect("failed to send key input event");
            });
        }
        {
            let sender = sender.clone();
            thread::spawn(move || loop {
                sender.send(Event::Tick).expect("failed to send tick event");
                thread::sleep(tick_rate);
            });
        }
        Self { sender, receiver }
    }

    /// Receives the next event from the channel.
    pub fn next(&self) -> Result<Event, RecvError> {
        self.receiver.recv()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::char;
    use std::time::Instant;

    const TICK_RATE_MS: u64 = 100;

    #[test]
    fn test_event() {
        let start_time = Instant::now();
        let event_handler = EventHandler::new(TICK_RATE_MS);
        let mut tick_count = 0;
        for i in 0..10 {
            let sender = event_handler.sender.clone();
            thread::spawn(move || {
                let key = Key::Char(char::from_digit(i, 10).unwrap_or('9'));
                let event = Event::KeyPress(key);
                sender.send(event).unwrap();
            });
            match event_handler.next().unwrap() {
                Event::KeyPress(key) => {
                    if key == Key::Char('9') {
                        break;
                    }
                }
                Event::Tick => {
                    thread::sleep(Duration::from_millis(TICK_RATE_MS));
                    tick_count += 1;
                }
            }
        }
        assert!(start_time.elapsed() > Duration::from_millis(tick_count * TICK_RATE_MS));
    }
}
