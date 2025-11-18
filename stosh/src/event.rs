use crossterm::event::Event as TerminalEvent;
use derive_more::From;
use tokio_command_multiplexer::ChildEvent;

#[derive(Debug, From)]
pub(crate) enum InputEvent {
    Terminal(TerminalEvent),
    Child(ChildEvent),
}

impl TryFrom<std::io::Result<TerminalEvent>> for InputEvent {
    type Error = std::io::Error;

    fn try_from(res: std::io::Result<TerminalEvent>) -> Result<Self, Self::Error> {
        res.map(InputEvent::Terminal)
    }
}
