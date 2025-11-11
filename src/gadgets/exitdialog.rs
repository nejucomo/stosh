#[cfg(test)]
mod tests;

use crossterm::event::{Event, KeyCode};
use derive_new::new;
use ratatui::layout::Constraint;
use ratatui::style::{Style, Stylize as _};
use ratatui::text::Text;
use ratatui::widgets::{Block, Clear, Padding};

use crate::{CenteredOverlay, ContextualWidget, RenderContext};
use crate::{EventHandler, Gadget, Notification::Exit, UI};

const BLOCK_BORDER_SIZE: u16 = 1;
const HORIZONTAL_PADDING: u16 = 2;
const VERTICAL_PADDING: u16 = 1;

/// The exit dialog
#[derive(Debug, new)]
pub struct ExitDialog(UI);

impl ExitDialog {}

impl Gadget for ExitDialog {}

impl ContextualWidget for &ExitDialog {
    fn render_to_context<'b>(self, ctx: &mut RenderContext<'b>) {
        let text = Text::styled("Exit? y/n", Style::new().bold().white().on_black());

        ctx.render(CenteredOverlay {
            horizontal: Constraint::Length(
                u16::try_from(text.width()).unwrap() + 2 * (HORIZONTAL_PADDING + BLOCK_BORDER_SIZE),
            ),

            vertical: Constraint::Length(
                u16::try_from(text.height()).unwrap() + 2 * (VERTICAL_PADDING + BLOCK_BORDER_SIZE),
            ),
        })
        .render(Clear)
        .render(Block::bordered().padding(Padding::symmetric(HORIZONTAL_PADDING, VERTICAL_PADDING)))
        .render(text);
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
                self.0.notify(Exit);
                Ok(true)
            }

            // Do not keep dialog open:
            Key(kev) if kev.code == KeyCode::Char('n') => Ok(false),

            // Ignore all other events:
            _ => Ok(true),
        }
    }
}
