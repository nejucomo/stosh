use crossterm::event::Event;

/// Types which can handle events
pub trait EventHandler {
    /// The type returned to the container when an event is handled
    type EventResult;

    /// Handle an incoming [Event]
    fn handle_event(&mut self, event: Event) -> std::io::Result<Self::EventResult>;
}
