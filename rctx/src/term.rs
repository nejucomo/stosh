use ratatui::{CompletedFrame, DefaultTerminal};

use crate::{RenderContext, Renderable};

/// Provides [RAII](https://en.wikipedia.org/wiki/Resource_acquisition_is_initialization) access to the terminal alternate screen; dropping causes a reset
#[derive(Debug)]
pub struct TerminalSession(DefaultTerminal);

impl TerminalSession {
    /// This initializes the session
    pub fn start() -> Self {
        TerminalSession(ratatui::init())
    }

    /// Draw onto a terminal
    pub fn draw<R>(&mut self, r: R) -> std::io::Result<CompletedFrame<'_>>
    where
        R: Renderable,
    {
        self.0
            .draw(|frame| r.render_into(RenderContext::new(frame.area(), frame.buffer_mut())))
    }
}

impl Drop for TerminalSession {
    fn drop(&mut self) {
        ratatui::restore();
    }
}
