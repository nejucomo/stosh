use crossterm::event::Event;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::WidgetRef;

use crate::gadgets::MainPane;
use crate::{EventHandler, Gadget};

/// The root [Gadget] encompassing all rendering and [Event] handling
///
/// In addition to simply wrapping the [MainPane], this gadget supports the `ExitDialog`
#[derive(Debug, Default)]
pub struct RootGadget {
    mp: MainPane,
    // dialog: Option<ExitDialog>,
}

impl Gadget for RootGadget {}

impl WidgetRef for RootGadget {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        self.mp.render_ref(area, buf);
    }
}

impl EventHandler for RootGadget {
    fn handle_event(&mut self, event: Event) -> std::io::Result<()> {
        self.mp.handle_event(event)
    }
}
