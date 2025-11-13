use crossterm::event::{Event, KeyCode};
use ratatui::layout::{Constraint, Flex};
use ratatui::style::{Style, Stylize as _};
use ratatui::text::Line;
use ratatui::widgets::{Block, BorderType, Borders, Clear, Padding, Widget};
use ratatui_rseq::{Renderable, RenderableSeq as _};

use crate::cmdinput::CommandInput;
use crate::handler::Handler;
use crate::u16util::IntoU16 as _;

#[derive(Debug, Default)]
pub(crate) struct UI {
    cmdinput: CommandInput,
    exitdialog: bool,
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
            .then(
                self.cmdinput
                    .constrained(Constraint::Length(self.cmdinput.height().into_u16()))
                    .on_top(),
            )
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

impl Handler<Event> for UI {
    type Response = std::io::Result<bool>;

    async fn handle(&mut self, ev: Event) -> Self::Response {
        if self.exitdialog {
            match ev {
                Event::Key(kev) if kev.code == KeyCode::Char('y') => Ok(false),
                Event::Key(kev) if kev.code == KeyCode::Char('n') => {
                    self.exitdialog = false;
                    Ok(true)
                }
                other => Err(std::io::Error::other(format!(
                    "unhandled exit dialog event: {other:#?}"
                ))),
            }
        } else if matches!(ev, Event::Key(kev) if kev.code == KeyCode::Esc) {
            self.exitdialog = true;
            Ok(true)
        } else if let Some(cmd) = self.cmdinput.handle(ev).await? {
            Err(std::io::Error::other(format!("handle cmd: {cmd:?}")))
        } else {
            Ok(true)
        }
    }
}
