use ratatui::DefaultTerminal;
use ratatui::style::Color;
use ratatui::widgets::{Block, BorderType, Borders};

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
            frame.render_widget(
                Block::new()
                    .title("══╡ partish ╞")
                    .title_style(Color::DarkGray)
                    .borders(Borders::TOP)
                    .border_type(BorderType::Double)
                    .border_style(Color::DarkGray),
                frame.area(),
            );
        })?;

        Ok(())
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        ratatui::restore();
    }
}
