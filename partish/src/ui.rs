use crossterm::event::Event;
use ratatui::style::{Style, Stylize as _};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Clear, Widget};
use ratatui_rctx::{Renderable, RenderableSeq as _};

use crate::cmdinput::CommandInput;
use crate::handler::Handler;

#[derive(Debug, Default)]
pub(crate) struct UI {
    cmdinput: CommandInput,
}

impl Renderable for &UI {
    fn into_widget(self) -> impl Widget {
        (
            Clear,
            Style::reset().gray().on_black(),
            Block::new()
                .title_top(Line::from("partish").light_green().right_aligned())
                .borders(Borders::TOP)
                .border_style(Style::new().green()),
        )
            .then(&self.cmdinput)
            .into_widget()
    }
}

impl Handler<Event> for UI {
    type Response = std::io::Result<()>;

    async fn handle(&mut self, ev: Event) -> Self::Response {
        self.cmdinput.handle(ev).await
    }
}
