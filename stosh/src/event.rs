mod fromimpls;

use std::process::ExitStatus;

use crossterm::event::Event as TerminalEvent;
use derive_more::From;
use derive_new::new;
use tokio_command_multiplexer::ChildEvent;

use crate::cmd;

#[derive(Debug, From)]
pub(crate) enum InputEvent {
    Terminal(TerminalEvent),
    #[from(ChildEvent<cmd::Handle>)]
    Child(CommandEvent),
}

#[derive(Debug, new)]
pub struct CommandEvent {
    pub(crate) handle: cmd::Handle,
    pub(crate) info: CommandEventInfo,
}

#[derive(Debug)]
pub enum CommandEventInfo {
    Spawn(std::io::Result<()>),
    Stdout(String),
    Stderr(String),
    Done(std::io::Result<ExitStatus>),
}

#[derive(Debug, From)]
pub(crate) enum ControlMessage {
    NoCtrl,
    Exit,
    LaunchCommand(usize, String),
}
