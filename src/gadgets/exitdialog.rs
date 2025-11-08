#[cfg(test)]
mod tests;

use crossterm::event::{Event, KeyCode};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Rect};
use ratatui::style::{Style, Stylize as _};
use ratatui::text::Text;
use ratatui::widgets::{Block, Clear, Padding, WidgetRef};

use crate::{EventHandler, Gadget, Notification::Exit, RectExt as _, UI};

const BLOCK_BORDER_SIZE: u16 = 1;
const HORIZONTAL_PADDING: u16 = 2;
const VERTICAL_PADDING: u16 = 1;

/// The exit dialog
#[derive(Debug)]
pub struct ExitDialog {
    ui: UI,
    block: Block<'static>,
    text: Text<'static>,
}

impl ExitDialog {
    /// Construct an exit dialog
    pub fn new(ui: UI) -> Self {
        ExitDialog {
            ui,
            block: Block::bordered()
                .padding(Padding::symmetric(HORIZONTAL_PADDING, VERTICAL_PADDING)),
            text: Text::styled("Exit? y/n", Style::new().bold().white().on_black()),
        }
    }

    fn horizontal_constraint(&self) -> Constraint {
        Constraint::Length(
            u16::try_from(self.text.width()).unwrap()
                + 2 * (HORIZONTAL_PADDING + BLOCK_BORDER_SIZE),
        )
    }

    fn vertical_constraint(&self) -> Constraint {
        Constraint::Length(
            u16::try_from(self.text.height()).unwrap() + 2 * (VERTICAL_PADDING + BLOCK_BORDER_SIZE),
        )
    }
}

impl Gadget for ExitDialog {}

impl WidgetRef for ExitDialog {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let area = area.centered_subrect(self.horizontal_constraint(), self.vertical_constraint());

        Clear.render_ref(area, buf);
        self.block.render_ref(area, buf);
        self.text.render_ref(self.block.inner(area), buf);
    }
}

impl EventHandler for ExitDialog {
    // Whether or not to keep the dialog open:
    type EventResult = bool;

    fn handle_event(&mut self, event: Event) -> std::io::Result<bool> {
        use Event::Key;

        match event {
            // Notify exit, and keep dialog open
            Key(kev) if kev.code == KeyCode::Char('y') => {
                self.ui.notify(Exit);
                Ok(true)
            }

            // Do not keep dialog open:
            Key(kev) if kev.code == KeyCode::Char('n') => Ok(false),

            // Ignore all other events:
            _ => Ok(true),
        }
    }
}
