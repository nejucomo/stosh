use crossterm::event::Event;
use ratatui::layout::Constraint::Length;
use ratatui::style::{Style, Stylize as _};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Widget};
use ratatui_rseq::{Renderable, RenderableSeq as _};

use crate::cmdinput::CommandInput;
use crate::handler::Handler;
use crate::u16util::IntoU16 as _;

#[derive(Debug, Default)]
pub(crate) struct MainScreen {
    cmdinput: CommandInput,
}

impl Renderable for &MainScreen {
    fn into_widget(self) -> impl Widget {
        Block::new()
            .title_top(Line::from("partish").light_green().right_aligned())
            .borders(Borders::TOP)
            .border_style(Style::new().green())
            .then(
                self.cmdinput
                    .constrained(Length(self.cmdinput.height().into_u16()))
                    .on_top(),
            )
    }
}

impl Handler<Event> for MainScreen {
    type Response = std::io::Result<bool>;

    async fn handle(&mut self, ev: Event) -> Self::Response {
        if let Some(cmd) = self.cmdinput.handle(ev).await? {
            Err(std::io::Error::other(format!("handle cmd: {cmd:?}")))
        } else {
            Ok(true)
        }
    }
}
