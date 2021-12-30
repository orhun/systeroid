use std::io;
use std::sync::mpsc::{self, Receiver, RecvError};
use std::thread;
use termion::event::Key;
use termion::input::TermRead;

/// Representation of terminal events.
#[derive(Clone, Copy, Debug)]
pub enum Event {
    /// A key press event.
    KeyPress(Key),
}

/// [`Event`] handler.
#[derive(Debug)]
pub struct EventHandler {
    /// Receiver channel for the events.
    receiver: Receiver<Event>,
}

impl Default for EventHandler {
    fn default() -> Self {
        let (sender, receiver) = mpsc::channel();
        thread::spawn(move || loop {
            io::stdin()
                .keys()
                .flatten()
                .try_for_each(|key| sender.send(Event::KeyPress(key)))
                .expect("failed to send key input event");
        });
        Self { receiver }
    }
}

impl EventHandler {
    /// Receives the next event from the channel.
    pub fn next(&self) -> Result<Event, RecvError> {
        self.receiver.recv()
    }
}
