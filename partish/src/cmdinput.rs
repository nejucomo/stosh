use crossterm::event::{Event, KeyCode};
use ratatui::layout::Constraint::{Fill, Length};
use ratatui::style::{Style, Stylize};
use ratatui::text::Line;
use ratatui_rctx::{RenderContext, Renderable};
use tui_textarea::TextArea;

use crate::handler::Handler;

#[derive(Debug)]
pub(crate) struct CommandInput {
    ta: TextArea<'static>,
}

impl Default for CommandInput {
    fn default() -> Self {
        let mut ta = TextArea::default();
        ta.set_cursor_style(Style::reset().on_light_cyan());
        ta.set_cursor_line_style(Style::default());
        ta.set_style(Style::reset().gray().on_black());

        Self { ta }
    }
}

impl Renderable for &CommandInput {
    fn render_into<'b>(self, rctx: RenderContext<'b>) {
        let mut prompt = Line::default();
        prompt.push_span("⟨0⟩".black().on_light_cyan());
        prompt.push_span(" ".black().on_black());
        let pwidth = prompt.width().try_into().unwrap();

        prompt
            .constrained(Length(pwidth))
            .on_left()
            .followed_by(self.ta.constrained(Fill(1)))
            .render_into(rctx);
    }
}

impl Handler<Event> for CommandInput {
    type Response = std::io::Result<()>;

    async fn handle(&mut self, ev: Event) -> Self::Response {
        match ev {
            Event::Key(kev) => {
                if kev.code == KeyCode::Enter {
                    Err(std::io::Error::other("not implemented: cmd input"))
                } else {
                    self.ta.input(ev);
                    Ok(())
                }
            }
            other => Err(std::io::Error::other(format!(
                "not implemented: {other:#?}"
            ))),
        }
    }
}
