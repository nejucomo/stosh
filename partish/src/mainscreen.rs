use crossterm::event::Event;
use ratatui::layout::Constraint::{Fill, Length};
use ratatui::style::{Style, Stylize as _};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Widget};
use ratatui_rseq::{Renderable, RenderableSeq as _};

use crate::cmd;
use crate::handler::Handler;
use crate::u16util::IntoU16 as _;

#[derive(Debug, Default)]
pub(crate) struct MainScreen {
    input: cmd::Input,
    stack: cmd::Stack,
}

impl Renderable for &MainScreen {
    fn into_widget(self) -> impl Widget {
        Block::new()
            .title_top(Line::from("partish").light_green().right_aligned())
            .borders(Borders::TOP)
            .border_style(Style::new().green())
            .then(&self.input)
            .constrained(Length(self.input.height().into_u16()))
            .on_top()
            .followed_by(self.stack.constrained(Fill(1)))
    }
}

impl Handler<Event> for MainScreen {
    type Response = std::io::Result<bool>;

    async fn handle(&mut self, ev: Event) -> Self::Response {
        if let Some(input) = self.input.handle(ev).await? {
            self.stack.handle_new_input(input)?;
        }
        Ok(true)
    }
}
