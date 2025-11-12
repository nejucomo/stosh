use ratatui::layout::Alignment::{Center, Right};
use ratatui::style::Stylize as _;
use ratatui::text::{Line, Text};
use ratatui::widgets::Borders;
use ratatui_rctx::{Renderable as _, TerminalSession};

fn main() -> std::io::Result<()> {
    let mut term = TerminalSession::start();
    term.draw(
        Text::raw("Hello World!")
            .within_block()
            .title_top(Line::from("rctx demo".blue().on_white()).alignment(Center))
            .title_bottom(
                Line::from("bottom right title".dark_gray().on_light_cyan()).alignment(Right),
            )
            .borders(Borders::all()),
    )?;
    ratatui::crossterm::event::read()?;
    Ok(())
}
