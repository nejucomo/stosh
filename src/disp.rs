use ratatui::DefaultTerminal;
use ratatui::widgets::Block;

/// Drive the output display
#[derive(Debug)]
pub struct Display {
    term: DefaultTerminal,
}

impl Display {
    /// Create a [Display]; this also activates the terminal as per [ratatui::init]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let term = ratatui::init();
        Display { term }
    }

    /// Draw the current frame
    pub fn draw(&mut self) -> std::io::Result<()> {
        self.term.draw(|frame| {
            frame.render_widget(Block::bordered(), frame.area());
        })?;

        Ok(())
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        ratatui::restore();
    }
}
