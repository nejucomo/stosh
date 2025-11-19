use crossterm::event::Event as TerminalEvent;
use tokio_command_multiplexer::ChildEvent;
use tokio_process_stream::Item as ChildStreamItem;

use crate::cmd;
use crate::event::{CommandEvent, CommandEventInfo, InputEvent};

impl TryFrom<std::io::Result<TerminalEvent>> for InputEvent {
    type Error = std::io::Error;

    fn try_from(res: std::io::Result<TerminalEvent>) -> Result<Self, Self::Error> {
        res.map(InputEvent::Terminal)
    }
}

impl From<ChildEvent<cmd::Handle>> for CommandEvent {
    fn from(chev: ChildEvent<cmd::Handle>) -> Self {
        CommandEvent {
            handle: chev.userdata,
            info: CommandEventInfo::from(chev.item),
        }
    }
}

impl From<ChildStreamItem<String>> for CommandEventInfo {
    fn from(item: ChildStreamItem<String>) -> Self {
        use ChildStreamItem;

        match item {
            ChildStreamItem::Stdout(x) => Self::Stdout(x),
            ChildStreamItem::Stderr(x) => Self::Stderr(x),
            ChildStreamItem::Done(x) => Self::Done(x),
        }
    }
}
