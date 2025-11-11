use crossterm::event::Event;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::WidgetRef;

use crate::{EventHandler, Gadget, UI};

/// The command entry gadget
#[derive(Debug)]
pub struct CommandEntry {
    #[allow(dead_code)]
    ui: UI,
}

impl CommandEntry {
    /// Construct with the [UI] notifier
    pub fn new(ui: UI) -> Self {
        Self { ui }
    }
}

impl Gadget for CommandEntry {}

impl WidgetRef for CommandEntry {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let _ = (area, buf);
        todo!()
    }
}

impl EventHandler for CommandEntry {
    type EventResult = Option<String>;

    fn handle_event(&mut self, event: Event) -> std::io::Result<Self::EventResult> {
        todo!("{event:#?}")
    }
}
