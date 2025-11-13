use ratatui::crossterm::ExecutableCommand as _;
use ratatui::crossterm::event::{
    KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
};
use ratatui::{CompletedFrame, DefaultTerminal};

use crate::Renderable;

/// Provides [RAII](https://en.wikipedia.org/wiki/Resource_acquisition_is_initialization) access to the terminal alternate screen; dropping causes a reset
#[derive(Debug)]
pub struct TerminalSession(DefaultTerminal);

impl TerminalSession {
    /// This initializes the session
    pub fn start() -> std::io::Result<Self> {
        let me = TerminalSession(ratatui::init());
        std::io::stdout().execute(PushKeyboardEnhancementFlags(
            KeyboardEnhancementFlags::REPORT_EVENT_TYPES,
        ))?;

        Ok(me)
    }

    /// Draw onto a terminal
    pub fn draw<R>(&mut self, r: R) -> std::io::Result<CompletedFrame<'_>>
    where
        R: Renderable,
    {
        self.0
            .draw(|frame| frame.render_widget(r.into_widget(), frame.area()))
    }
}

impl Drop for TerminalSession {
    fn drop(&mut self) {
        std::io::stdout()
            .execute(PopKeyboardEnhancementFlags)
            .unwrap();
        ratatui::restore();
    }
}
