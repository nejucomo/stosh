use crossterm::event::KeyEvent;
use crossterm::event::{Event::Key, KeyCode, KeyEventKind::Press};
use ratatui::layout::{Constraint, Flex};
use ratatui::style::{Style, Stylize as _};
use ratatui::text::Line;
use ratatui::widgets::{Block, BorderType, Clear, Padding, Widget};
use ratatui_rseq::{Renderable, RenderableSeq as _};

use crate::event::{ControlMessage, InputEvent};
use crate::handler::Handler;
use crate::mainscreen::MainScreen;
use crate::u16util::IntoU16 as _;

#[derive(Debug, Default)]
pub(crate) struct UI {
    ms: MainScreen,
    exitdialog: bool,
}

impl Renderable for &UI {
    fn into_widget(self) -> impl Widget {
        (Clear, Style::reset().gray().on_black())
            .then(&self.ms)
            .then(if self.exitdialog {
                let line = Line::from("Exit? y/n").bold().white();
                let width = (line.width() + 6).into_u16();
                let height = 5;

                Some(
                    (
                        Clear,
                        Block::bordered()
                            .border_type(BorderType::Double)
                            .padding(Padding::symmetric(2, 1))
                            .style(Style::reset().on_blue()),
                    )
                        .then(line)
                        .constrained(Constraint::Length(width))
                        .on_left()
                        .flex(Flex::Center)
                        .constrained(Constraint::Length(height))
                        .on_top()
                        .flex(Flex::Center),
                )
            } else {
                None
            })
            .into_widget()
    }
}

impl Handler<InputEvent> for UI {
    type Response = ControlMessage;

    fn handle(&mut self, ev: InputEvent) -> std::io::Result<ControlMessage> {
        use ControlMessage::{Exit, NoCtrl};
        use InputEvent::*;

        // ignore key event kind besides Press:
        if matches!(ev, Terminal(Key(KeyEvent { kind, .. })) if kind != Press) {
            Ok(NoCtrl)
        } else if self.exitdialog {
            match ev {
                Terminal(Key(kev)) if kev.code == KeyCode::Char('y') => Ok(Exit),
                Terminal(Key(kev)) if kev.code == KeyCode::Char('n') => {
                    self.exitdialog = false;
                    Ok(NoCtrl)
                }
                other => Err(std::io::Error::other(format!(
                    "unhandled exit dialog event: {other:#?}"
                ))),
            }
        } else if matches!(ev, Terminal(Key(kev)) if kev.code == KeyCode::Esc) {
            self.exitdialog = true;
            Ok(NoCtrl)
        } else {
            self.ms.handle(ev)
        }
    }
}
