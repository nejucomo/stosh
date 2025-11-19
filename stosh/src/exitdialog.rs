use crossterm::event::Event::{self as TerminalEvent, Key};
use crossterm::event::KeyCode;
use ratatui::layout::{Constraint, Flex};
use ratatui::style::{Style, Stylize as _};
use ratatui::text::Line;
use ratatui::widgets::{Block, BorderType, Clear, Padding, Widget};
use ratatui_rseq::{Renderable, RenderableSeq as _};

use crate::event::ControlMessage;
use crate::handler::Handler;
use crate::u16util::IntoU16 as _;

#[derive(Copy, Clone, Debug)]
pub struct ExitDialog;

impl Handler<TerminalEvent> for ExitDialog {
    type Response = ControlMessage;

    #[tracing::instrument]
    fn handle(&mut self, ev: TerminalEvent) -> Self::Response {
        use ControlMessage::*;

        let ctl = match ev {
            Key(kev) if kev.code == KeyCode::Char('y') => Exit,
            Key(kev) if kev.code == KeyCode::Char('n') => NoCtrl,
            unhandled => {
                tracing::debug!(?unhandled);
                NoCtrl
            }
        };
        tracing::debug!(?ctl);
        ctl
    }
}

impl Renderable for ExitDialog {
    fn into_widget(self) -> impl Widget {
        let line = Line::from("Exit? y/n").bold().white();
        let width = (line.width() + 6).into_u16();
        let height = 5;

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
            .flex(Flex::Center)
    }
}
