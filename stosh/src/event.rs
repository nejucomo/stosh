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
    #[from(CommandEvent, ChildEvent<cmd::Handle>)]
    Child(CommandEvent),
}

#[derive(Debug, new)]
pub struct CommandEvent {
    pub(crate) handle: cmd::Handle,
    #[new(into)]
    pub(crate) info: CommandEventInfo,
}

#[derive(Debug, From)]
pub enum CommandEventInfo {
    #[from]
    Spawn(std::io::Result<()>),
    Stdout(String),
    Stderr(String),
    #[from]
    Done(std::io::Result<ExitStatus>),
}

#[derive(Debug, From)]
pub(crate) enum ControlMessage {
    NoCtrl,
    Exit,
    LaunchCommand(usize, Vec<String>),
}
