use crossterm::event::Event;
use ratatui::text::Text;

use crate::{ContextualWidget, EventHandler, Gadget, RenderContext, UI};

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

impl ContextualWidget for &CommandEntry {
    fn render_to_context<'b>(self, ctx: &mut RenderContext<'b>) {
        ctx.render(Text::raw("TODO: CommandEntry"));
    }
}

impl EventHandler for CommandEntry {
    type EventResult = Option<String>;

    fn handle_event(&mut self, event: Event) -> std::io::Result<Self::EventResult> {
        todo!("{event:#?}")
    }
}
