use derive_new::new;
use futures::Stream;
use pin_project::pin_project;
use tokio_process_stream::{Item, ProcessLineStream as Inner};

use crate::Handle;

/// A wrapper around [tokio_process_stream::ProcessLineStream] which also provides a [Handle]
#[derive(Debug, new)]
#[new(visibility = "pub(crate)")]
#[pin_project]
pub struct ProcessLineStream {
    handle: Handle,
    #[new(into)]
    #[pin]
    inner: Inner,
}

impl Stream for ProcessLineStream {
    type Item = (Handle, Item<String>);

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let handle = self.handle;
        self.project()
            .inner
            .poll_next(cx)
            .map(|optitem| optitem.map(|item| (handle, item)))
    }
}
