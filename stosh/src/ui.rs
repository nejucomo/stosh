use crossterm::event::KeyEvent;
use crossterm::event::{Event::Key, KeyCode, KeyEventKind::Press};
use ratatui::style::{Style, Stylize as _};
use ratatui::widgets::{Clear, Widget};
use ratatui_rseq::{Renderable, RenderableSeq as _};

use crate::event::{ControlMessage, InputEvent};
use crate::exitdialog::ExitDialog;
use crate::handler::Handler;
use crate::mainscreen::MainScreen;

#[derive(Debug, Default)]
pub(crate) struct UI {
    ms: MainScreen,
    exitdialog: Option<ExitDialog>,
}

impl Handler<InputEvent> for UI {
    type Response = ControlMessage;

    fn handle(&mut self, ev: InputEvent) -> ControlMessage {
        use ControlMessage::NoCtrl;
        use InputEvent::*;

        match ev {
            Terminal(ev) => {
                // ignore key event kind besides Press:
                if matches!(ev, Key(KeyEvent { kind, .. }) if kind != Press) {
                    NoCtrl
                } else if let Some(mut dialog) = self.exitdialog {
                    dialog.handle(ev)
                } else if matches!(ev, Key(kev) if kev.code == KeyCode::Esc) {
                    self.exitdialog = Some(ExitDialog);
                    NoCtrl
                } else {
                    self.ms.handle(Terminal(ev))
                }
            }
            Child(ev) => self.ms.handle(Child(ev)),
        }
    }
}

impl Renderable for &UI {
    fn into_widget(self) -> impl Widget {
        (Clear, Style::reset().gray().on_black())
            .then(&self.ms)
            .then(self.exitdialog)
            .into_widget()
    }
}
