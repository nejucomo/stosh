use crossterm::event::{Event, KeyCode};
use ratatui::layout::Constraint::{Fill, Length};
use ratatui::style::{Style, Stylize};
use ratatui::text::Line;
use ratatui::widgets::Widget;
use ratatui_rseq::Renderable;
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
        ta.set_style(Style::reset().gray().on_dark_gray());

        Self { ta }
    }
}

impl Renderable for &CommandInput {
    fn into_widget(self) -> impl Widget {
        let prompt = Line::from("⟨0⟩".black().on_light_cyan());
        let pwidth = prompt.width().try_into().unwrap();

        prompt
            .constrained(Length(pwidth))
            .on_left()
            .followed_by(self.ta.constrained(Fill(1)))
            .horizontal_margin(1)
            .spacing(1)
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
