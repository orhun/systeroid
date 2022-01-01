use std::io;
use std::sync::mpsc::{self, Receiver, RecvError};
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
        thread::spawn(move || loop {
            sender.send(Event::Tick).expect("failed to send tick event");
            thread::sleep(tick_rate);
        });
        Self { receiver }
    }

    /// Receives the next event from the channel.
    pub fn next(&self) -> Result<Event, RecvError> {
        self.receiver.recv()
    }
}
