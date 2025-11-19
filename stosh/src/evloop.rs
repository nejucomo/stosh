use std::collections::VecDeque;
use std::ffi::OsStr;

use crossterm::event::EventStream;
use futures::{StreamExt as _, stream};
use ratatui_rseq::TerminalSession;
use tokio::process::Command;
use tokio_command_multiplexer::CommandMultiplexer;

use crate::cmd;
use crate::event::{CommandEvent, ControlMessage, InputEvent};
use crate::handler::Handler;
use crate::ui::UI;

#[derive(Debug)]
struct EventLoop {
    ui: UI,
    termev: EventStream,
    cmux: CommandMultiplexer<cmd::Handle>,
    inq: VecDeque<InputEvent>,
}

pub(crate) async fn run() -> std::io::Result<()> {
    let mut evloop = EventLoop {
        ui: UI::default(),
        termev: EventStream::new(),
        cmux: CommandMultiplexer::default(),
        inq: VecDeque::default(),
    };

    let mut term = TerminalSession::start()?;

    term.draw(&evloop.ui)?;
    while let Some(ev) = evloop.next_event().await? {
        let ctrlmsg = evloop.ui.handle(ev);
        if matches!(ctrlmsg, ControlMessage::Exit) {
            break;
        } else if let Some(ev) = evloop.handle(ctrlmsg) {
            evloop.inq.push_back(ev);
        }
        term.draw(&evloop.ui)?;
    }
    Ok(())
}

impl EventLoop {
    #[tracing::instrument]
    async fn next_event(&mut self) -> std::io::Result<Option<InputEvent>> {
        let evres = if let Some(inp) = self.inq.pop_front() {
            Ok(Some(inp))
        } else {
            stream::select(
                (&mut self.termev).map(InputEvent::try_from),
                (&mut self.cmux).map(InputEvent::from).map(Ok),
            )
            .next()
            .await
            .transpose()
        };

        tracing::debug!(?evres);
        evres
    }

    fn parse_and_spawn<I, S>(&mut self, h: cmd::Handle, cmdargs: I) -> std::io::Result<()>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let mut cmdargs = cmdargs.into_iter();
        let cmd = cmdargs
            .next()
            .ok_or_else(|| std::io::Error::other("no command given"))?;

        self.cmux.spawn(h, Command::new(cmd.as_ref()).args(cmdargs))
    }
}

impl Handler<ControlMessage> for EventLoop {
    type Response = Option<InputEvent>;

    fn handle(&mut self, ctrlmsg: ControlMessage) -> Option<InputEvent> {
        use ControlMessage::*;

        match ctrlmsg {
            NoCtrl => None,
            Exit => panic!("Remove this case with the type system."),
            LaunchCommand(h, cmdlines) => {
                let res = self.parse_and_spawn(h, cmdlines);
                Some(CommandEvent::new(h, res).into())
            }
        }
    }
}
