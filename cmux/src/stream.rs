use derive_new::new;
use futures::Stream;
use pin_project::pin_project;
use tokio_process_stream::ProcessLineStream as Inner;

use crate::ChildEvent;

/// A wrapper around [tokio_process_stream::ProcessLineStream] which also provides clone of `T` userdata associated with the child
#[derive(Debug, new)]
#[new(visibility = "pub(crate)")]
#[pin_project]
pub struct ProcessLineStream<T>
where
    T: Clone,
{
    userdata: T,
    #[new(into)]
    #[pin]
    inner: Inner,
}

impl<T> Stream for ProcessLineStream<T>
where
    T: Clone,
{
    type Item = ChildEvent<T>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let userdata = self.userdata.clone();
        self.project()
            .inner
            .poll_next(cx)
            .map(|optitem| optitem.map(|item| ChildEvent { userdata, item }))
    }
}
