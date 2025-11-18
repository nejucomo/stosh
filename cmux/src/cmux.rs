use std::ffi::OsStr;

use derive_debug::Dbg;
use futures::{Stream, stream};
use pin_project::pin_project;
use tokio::process;
use tokio_process_stream::Item;

use crate::handle::HandleAllocator;
use crate::stream::ProcessLineStream;
use crate::{Command, Handle};

/// Interleaving subprocess I/O within a single task
#[derive(Dbg, Default)]
#[pin_project]
pub struct CommandMultiplexer {
    halloc: HandleAllocator,
    #[dbg(placeholder = "…")]
    #[pin]
    sa: stream::SelectAll<ProcessLineStream>,
}

impl CommandMultiplexer {
    /// Construct a new [Command]
    pub fn cmd<S>(&mut self, program: S) -> Command<'_>
    where
        S: AsRef<OsStr>,
    {
        let handle = self.halloc.next();
        Command::new(self, handle, process::Command::new(program))
    }

    pub(crate) fn spawn_inner(
        &mut self,
        h: Handle,
        mut innercmd: process::Command,
    ) -> std::io::Result<Handle> {
        let child = innercmd.spawn()?;
        self.sa.push(ProcessLineStream::new(h, child));
        Ok(h)
    }
}

impl Stream for CommandMultiplexer {
    type Item = (Handle, Item<String>);

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.project().sa.poll_next(cx)
    }
}
