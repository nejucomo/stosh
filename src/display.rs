use crossterm::event::Event;
use ratatui::DefaultTerminal;

use crate::{EventHandler, gadgets};

/// Drive the output display
#[derive(Debug)]
pub struct Display {
    term: DefaultTerminal,
    mp: gadgets::MainPane,
}

impl Display {
    /// Create a [Display]; this also activates the terminal as per [ratatui::init]
    pub fn start() -> Self {
        let term = ratatui::init();
        Display {
            term,
            mp: gadgets::MainPane::default(),
        }
    }

    /// Draw the current frame
    pub fn draw(&mut self) -> std::io::Result<()> {
        self.term
            .draw(|frame| frame.render_widget(&self.mp, frame.area()))?;
        Ok(())
    }
}

impl EventHandler for Display {
    fn handle_event(&mut self, event: Event) -> std::io::Result<()> {
        use Event::*;

        match event {
            // FocusGained => todo!(),
            // FocusLost => todo!(),
            Key(kev) => self.mp.handle_event(Key(kev)),
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
