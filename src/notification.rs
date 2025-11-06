use derive_more::From;

/// A [Notification] is a one-way messages to the UI control thread
#[derive(Debug, From)]
pub enum Notification {
    /// Propagate an unhandled error
    ThreadError(std::io::Error),
    /// A UI input [Event](crossterm::event::Event)
    CrosstermEvent(crossterm::event::Event),
}
