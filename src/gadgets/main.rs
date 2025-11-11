use crossterm::event::Event;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Style, Stylize as _};
use ratatui::widgets::{Block, BorderType, Borders, Clear, WidgetRef};

use crate::gadgets::CommandEntry;
use crate::{EventHandler, Gadget, UI};

/// The main gadget for the whole UI
#[derive(Debug)]
pub struct MainPane {
    block: Block<'static>,
    cmdentry: CommandEntry,
}

impl MainPane {
    /// Construct with the [UI] notifier
    pub fn new(ui: UI) -> Self {
        Self {
            block: Block::new()
                .title("══╡ partish ╞")
                .title_style(Style::new().dark_gray())
                .borders(Borders::TOP)
                .border_type(BorderType::Double)
                .border_style(Style::new().dark_gray()),
            cmdentry: CommandEntry::new(ui),
        }
    }
}

impl Gadget for MainPane {}

impl WidgetRef for MainPane {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        Clear.render_ref(area, buf);
        self.block.render_ref(area, buf);
        self.cmdentry.render_ref(self.block.inner(area), buf);
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
