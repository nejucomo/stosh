use crossterm::event::Event;
use ratatui::style::{Style, Stylize as _};
use ratatui::text::Line;
use ratatui::widgets::Borders;
use ratatui_rctx::{RenderContext, Renderable};

use crate::cmdinput::CommandInput;
use crate::handler::Handler;

#[derive(Debug, Default)]
pub(crate) struct UI {
    cmdinput: CommandInput,
}

impl Renderable for &UI {
    fn render_into<'b>(self, rctx: RenderContext<'b>) {
        self.cmdinput
            .within_block()
            .title_top(Line::from("partish").light_green().right_aligned())
            .borders(Borders::TOP)
            .border_style(Style::new().green())
            .render_into(rctx);
    }
}

impl Handler<Event> for UI {
    type Response = std::io::Result<()>;

    async fn handle(&mut self, ev: Event) -> Self::Response {
        self.cmdinput.handle(ev).await
    }
}
