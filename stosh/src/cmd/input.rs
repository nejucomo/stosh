use crossterm::event::Event::{self, Key};
use crossterm::event::KeyCode::Enter;
use crossterm::event::{KeyEvent, KeyModifiers};
use ratatui::layout::Constraint::{Fill, Length};
use ratatui::style::{Style, Stylize};
use ratatui::text::Line;
use ratatui::widgets::Widget;
use ratatui_rseq::Renderable;

use crate::cmd::{Handle, TextArea};
use crate::event::ControlMessage;
use crate::handler::Handler;
use crate::prompt;
use crate::u16util::IntoU16 as _;

#[derive(Debug)]
pub(crate) struct Input {
    ta: TextArea,
    histix: Handle,
}

impl Input {
    /// The height of the CommandInput
    pub(crate) fn height(&self) -> usize {
        self.ta.height()
    }
}

impl Default for Input {
    fn default() -> Self {
        Input {
            ta: TextArea::default()
                .set_cursor_style(Style::default().on_blue())
                .set_style(Style::default().black().on_gray()),
            histix: 0,
        }
    }
}

impl Renderable for &Input {
    fn into_widget(self) -> impl Widget {
        let prompt = Line::from(prompt::text(self.histix).black().on_light_cyan());
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
    type Response = ControlMessage;

    fn handle(&mut self, ev: Event) -> std::io::Result<ControlMessage> {
        use ControlMessage::{LaunchCommand, NoCtrl};

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
                    return Ok(NoCtrl);
                };

                if self.height() > 1 {
                    // When we're already in multi-line mode, we invert the CONTROL meaning
                    send_cmd = !send_cmd;
                }

                Ok(if send_cmd {
                    let Input { histix, ta } = std::mem::take(self);
                    self.histix = histix + 1;
                    let mut cmd = "".to_string();
                    for line in ta.lines() {
                        cmd.push_str(line);
                        cmd.push('\n');
                    }
                    LaunchCommand(histix, cmd)
                } else {
                    self.ta.insert_newline();
                    NoCtrl
                })
            }

            // Forward all other events to `self.ta`:
            ev => {
                self.ta.handle(ev)?;
                Ok(NoCtrl)
            }
        }
    }
}
