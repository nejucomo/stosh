use crossterm::event::Event;
use ratatui::layout::Constraint::{Fill, Length};
use ratatui::style::{Style, Stylize as _};
use ratatui::text::Line;
use ratatui::widgets::Borders;
use ratatui_rctx::{RenderContext, Renderable};
use tui_textarea::TextArea;

use crate::handler::Handler;

#[derive(Debug, Default)]
pub(crate) struct UI {
    cmdinput: TextArea<'static>,
}

impl Renderable for &UI {
    fn render_into<'b>(self, rctx: RenderContext<'b>) {
        let mut prompt = Line::default();
        prompt.push_span("⟨0⟩".black().on_light_cyan());
        prompt.push_span(" ".black().on_black());
        let pwidth = prompt.width().try_into().unwrap();

        prompt
            .constrained(Length(pwidth))
            .on_left()
            .followed_by(self.cmdinput.constrained(Fill(1)))
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
        Err(std::io::Error::other(format!("unhandled event: {ev:#?}")))
    }
}
