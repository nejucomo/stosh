use std::marker::PhantomData;
use std::process::Stdio;

use derive_debug::Dbg;
use futures::{Stream, stream};
use pin_project::pin_project;
use tokio::process::Command;

use crate::ChildEvent;
use crate::stream::ProcessLineStream;

/// Interleaving subprocess I/O within a single task
#[derive(Dbg, Default)]
#[pin_project]
pub struct CommandMultiplexer<T>
where
    T: Clone,
{
    #[dbg(placeholder = "…")]
    #[pin]
    sa: stream::SelectAll<ProcessLineStream<T>>,
    ph: PhantomData<T>,
}

impl<T> CommandMultiplexer<T>
where
    T: Clone,
{
    /// Spawn a child
    pub fn spawn(&mut self, userdata: T, cmd: &mut Command) -> std::io::Result<()> {
        let child = cmd
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        self.sa.push(ProcessLineStream::new(userdata, child));
        Ok(())
    }
}

impl<T> Stream for CommandMultiplexer<T>
where
    T: Clone,
{
    type Item = ChildEvent<T>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.project().sa.poll_next(cx)
    }
}
