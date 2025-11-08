use crossterm::event::Event;

/// Types which can handle events
pub trait EventHandler {
    /// Handle an incoming [Event]
    fn handle_event(&mut self, event: Event) -> std::io::Result<()>;
}
