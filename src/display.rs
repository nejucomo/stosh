use crossterm::event::Event;
use ratatui::DefaultTerminal;

use crate::gadgets::RootGadget;
use crate::{EventHandler, RenderContext, UI};

/// Drive the output display
#[derive(Debug)]
pub struct Display {
    term: DefaultTerminal,
    root: RootGadget,
}

impl Display {
    /// Create a [Display]; this also activates the terminal as per [ratatui::init]
    pub fn start(ui: UI) -> Self {
        let term = ratatui::init();
        Display {
            term,
            root: RootGadget::new(ui),
        }
    }

    /// Draw the current frame
    pub fn draw(&mut self) -> std::io::Result<()> {
        self.term.draw(|frame| {
            frame.render_widget(RenderContext::wrap_contextual(&self.root), frame.area())
        })?;
        Ok(())
    }
}

impl EventHandler for Display {
    type EventResult = ();

    fn handle_event(&mut self, event: Event) -> std::io::Result<()> {
        use Event::*;

        match event {
            // FocusGained => todo!(),
            // FocusLost => todo!(),
            Key(kev) => self.root.handle_event(Key(kev)),
            // Mouse(mouse_event) => todo!(),
            // Paste(_) => todo!(),
            Resize(_, _) => {
                // disp.draw already accounts for resizes
                Ok(())
            }

            other => todo!("unhandled: {other:?}"),
        }
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        ratatui::restore();
    }
}
