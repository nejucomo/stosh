use crossterm::event::Event::{self, Key};
use crossterm::event::KeyCode::Enter;
use crossterm::event::{KeyEvent, KeyModifiers};
use ratatui::layout::Constraint::{Fill, Length};
use ratatui::style::{Style, Stylize};
use ratatui::text::Line;
use ratatui::widgets::Widget;
use ratatui_rseq::Renderable;
use tui_textarea::TextArea;

use crate::handler::Handler;
use crate::u16util::IntoU16 as _;

#[derive(Debug)]
pub(crate) struct Input {
    ta: TextArea<'static>,
    histix: usize,
}

impl Input {
    /// The height of the CommandInput
    pub(crate) fn height(&self) -> usize {
        self.ta.lines().len()
    }
}

impl Default for Input {
    fn default() -> Self {
        Self {
            ta: {
                let mut ta = TextArea::default();
                ta.set_cursor_style(Style::reset().on_light_cyan());
                ta.set_cursor_line_style(Style::default());
                ta.set_style(Style::reset().gray().on_dark_gray());
                ta
            },
            histix: 0,
        }
    }
}

impl Renderable for &Input {
    fn into_widget(self) -> impl Widget {
        let prompt = Line::from(format!("⟨{}⟩", self.histix).black().on_light_cyan());
        let pwidth = prompt.width().into_u16();

        prompt
            .constrained(Length(pwidth))
            .on_left()
            .followed_by(self.ta.constrained(Fill(1)))
            .horizontal_margin(1)
            .spacing(1)
    }
}

impl Handler<Event> for Input {
    type Response = std::io::Result<Option<Self>>;

    async fn handle(&mut self, ev: Event) -> Self::Response {
        match ev {
            Key(KeyEvent {
                code: Enter,
                modifiers,
                ..
            }) => {
                let mut send_cmd = if modifiers.is_empty() {
                    true
                } else if modifiers == KeyModifiers::CONTROL {
                    false
                } else {
                    // We ignore any other modifiers on return
                    return Ok(None);
                };

                if self.height() > 1 {
                    // When we're already in multi-line mode, we invert the CONTROL meaning
                    send_cmd = !send_cmd;
                }

                Ok(if send_cmd {
                    Some(std::mem::take(self))
                } else {
                    self.ta.insert_newline();
                    None
                })
            }

            // Forward all other events to `self.ta`:
            ev => {
                self.ta.input(ev);
                Ok(None)
            }
        }
    }
}
