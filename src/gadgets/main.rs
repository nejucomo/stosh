use crossterm::event::Event;
use ratatui::style::{Style, Stylize as _};
use ratatui::widgets::{Block, BorderType, Borders};

use crate::gadgets::CommandEntry;
use crate::{ContextualWidget, EventHandler, Gadget, RenderContext, UI};

/// The main gadget for the whole UI
#[derive(Debug)]
pub struct MainPane {
    cmdentry: CommandEntry,
}

impl MainPane {
    /// Construct with the [UI] notifier
    pub fn new(ui: UI) -> Self {
        Self {
            cmdentry: CommandEntry::new(ui),
        }
    }
}

impl Gadget for MainPane {}

impl ContextualWidget for &MainPane {
    fn render_to_context<'b>(self, ctx: &mut RenderContext<'b>) {
        ctx.render(
            Block::new()
                .title("══╡ partish ╞")
                .title_style(Style::new().dark_gray())
                .borders(Borders::TOP)
                .border_type(BorderType::Double)
                .border_style(Style::new().dark_gray()),
        )
        .render(&self.cmdentry);
    }
}

impl EventHandler for MainPane {
    type EventResult = ();

    fn handle_event(&mut self, event: Event) -> std::io::Result<()> {
        if let Some(cmd) = self.cmdentry.handle_event(event)? {
            todo!("{cmd:?}");
        }
        Ok(())
    }
}
