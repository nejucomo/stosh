use crossterm::event::EventStream;
use futures::{StreamExt as _, stream};
use ratatui_rseq::TerminalSession;
use tokio::process::Command;
use tokio_command_multiplexer::CommandMultiplexer;

use crate::cmd;
use crate::event::{CommandEvent, ControlMessage, InputEvent};
use crate::handler::Handler;
use crate::ui::UI;

pub(crate) async fn run() -> std::io::Result<()> {
    let mut evloop = EventLoop {
        ui: UI::default(),
        termev: EventStream::new(),
        cmux: CommandMultiplexer::default(),
        cmdev: None,
    };

    let mut term = TerminalSession::start()?;

    term.draw(&evloop.ui)?;
    while let Some(ev) = evloop.next_event().await? {
        let ctrlmsg = evloop.ui.handle(ev)?;
        evloop.cmdev = evloop.handle(ctrlmsg)?;
        term.draw(&evloop.ui)?;
    }
    Ok(())
}

struct EventLoop {
    ui: UI,
    termev: EventStream,
    cmux: CommandMultiplexer<cmd::Handle>,
    cmdev: Option<CommandEvent>,
}

impl EventLoop {
    async fn next_event(&mut self) -> std::io::Result<Option<InputEvent>> {
        // Pop any pending command event:
        if let Some(cmdev) = self.cmdev.take() {
            Ok(Some(InputEvent::from(cmdev)))
        } else {
            stream::select(
                (&mut self.termev).map(InputEvent::try_from),
                (&mut self.cmux).map(InputEvent::from).map(Ok),
            )
            .next()
            .await
            .transpose()
        }
    }

    fn parse_and_spawn(&mut self, h: cmd::Handle, cmdstr: &str) -> std::io::Result<()> {
        let mut argsit = cmdstr.lines();
        let cmd = argsit
            .next()
            .ok_or_else(|| std::io::Error::other("no command given"))?;

        self.cmux.spawn(h, Command::new(cmd).args(argsit))
    }
}

impl Handler<ControlMessage> for EventLoop {
    type Response = Option<SpawnResult>;

    fn handle(&mut self, ctrlmsg: ControlMessage) -> std::io::Result<Self::Response> {
        use ControlMessage::*;

        match ctrlmsg {
            NoCtrl => Ok(None),
            Exit => Ok(None),
            LaunchCommand(h, cmd) => {
                let res = self.parse_and_spawn(h, &cmd);
                Ok(Some(SpawnResult(h, res)))
            }
        }
    }
}
