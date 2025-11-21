use crossterm::event::Event::{self, Key};
use crossterm::event::KeyCode::Enter;
use crossterm::event::{KeyEvent, KeyModifiers};
use ratatui::style::Styled as _;
use ratatui::widgets::{Padding, Widget};
use ratatui_rseq::Renderable;

use crate::cmd::{Handle, TextArea};
use crate::cuteblock::CuteBlock;
use crate::event::ControlMessage;
use crate::handler::Handler;
use crate::prompt;
use crate::styles::STYLES;

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

    fn cmd_name(&self) -> &str {
        self.ta
            .lines()
            .iter()
            .flat_map(|l| l.split(' '))
            .find(|w| !w.trim().is_empty())
            .unwrap_or("•")
    }
}

impl Default for Input {
    fn default() -> Self {
        Input {
            ta: TextArea::default()
                .set_cursor_style(STYLES.text.input)
                .set_style(STYLES.text.input),
            histix: 0,
        }
    }
}

impl Renderable for &Input {
    fn into_widget(self) -> impl Widget {
        let st = STYLES.text.histix;

        CuteBlock::bordered()
            .title_top(prompt::text(self.histix).set_style(st))
            .title_top(self.cmd_name().set_style(st))
            .border_style(STYLES.border.input.style)
            .border_type(STYLES.border.input.btype)
            .padding(Padding::horizontal(1))
    }
}

impl Handler<Event> for Input {
    type Response = ControlMessage;

    fn handle(&mut self, ev: Event) -> ControlMessage {
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
                    return NoCtrl;
                };

                if self.height() > 1 {
                    // When we're already in multi-line mode, we invert the CONTROL meaning
                    send_cmd = !send_cmd;
                }

                if send_cmd {
                    let Input { histix, ta } = std::mem::take(self);
                    self.histix = histix + 1;
                    LaunchCommand(histix, ta.into_lines())
                } else {
                    self.ta.insert_newline();
                    NoCtrl
                }
            }

            // Forward all other events to `self.ta`:
            ev => {
                self.ta.handle(ev);
                NoCtrl
            }
        }
    }
}
