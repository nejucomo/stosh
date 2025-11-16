use crossterm::event::Event::{self, Key};
use crossterm::event::KeyCode::Enter;
use crossterm::event::{KeyEvent, KeyModifiers};
use ratatui::layout::Constraint::{Fill, Length};
use ratatui::style::{Style, Stylize};
use ratatui::text::Line;
use ratatui::widgets::Widget;
use ratatui_rseq::Renderable;

use crate::cmd::TextArea;
use crate::handler::Handler;
use crate::u16util::IntoU16 as _;

#[derive(Default, Debug)]
pub(crate) struct Input {
    ta: TextArea,
    histix: usize,
}

impl Input {
    /// The height of the CommandInput
    pub(crate) fn height(&self) -> usize {
        self.ta.height()
    }
}

impl Renderable for &Input {
    fn into_widget(self) -> impl Widget {
        let prompt = Line::from(format!("⟨{}⟩", self.histix).black().on_light_cyan());
        let pwidth = prompt.width().into_u16();

        prompt
            .constrained(Length(pwidth))
            .on_left("prompt")
            .followed_by("ta", self.ta.constrained(Fill(1)))
            .horizontal_margin(1)
            .spacing(1)
    }
}

impl Handler<Event> for Input {
    type Response = std::io::Result<Option<TextArea>>;

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
                    Some(std::mem::take(&mut self.ta))
                } else {
                    self.ta.insert_newline();
                    None
                })
            }

            // Forward all other events to `self.ta`:
            ev => {
                self.ta.handle(ev).await;
                Ok(None)
            }
        }
    }
}
