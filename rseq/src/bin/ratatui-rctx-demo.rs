use ratatui::layout::Alignment::{Center, Right};
use ratatui::style::Stylize as _;
use ratatui::text::{Line, Text};
use ratatui::widgets::Block;
use ratatui_rseq::{RenderableSeq as _, TerminalSession};

fn main() -> std::io::Result<()> {
    let mut term = TerminalSession::start();
    term.draw(
        Block::bordered()
            .title_top(Line::from("rseq demo".blue().on_white()).alignment(Center))
            .title_bottom(
                Line::from("bottom right title".dark_gray().on_light_cyan()).alignment(Right),
            )
            .then(Text::raw("Hello World!")),
    )?;
    ratatui::crossterm::event::read()?;
    Ok(())
}
