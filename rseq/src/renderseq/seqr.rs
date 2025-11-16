use debug_rollup::{DebugRollup, delegate_debug_to_rollup};
use derive_new::new;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::Widget;

use crate::{Renderable, RenderableSeq};

/// A [Renderable] which encapsulates a sequence produced by [RenderableSeq::then]
#[derive(new)]
pub struct SeqRenderable<T, R>
where
    T: RenderableSeq,
    R: Renderable,
{
    init: T,
    r: R,
}

delegate_debug_to_rollup!(
    SeqRenderable<T, R>
    where
        T: RenderableSeq,
        R: Renderable
);

impl<T, R> RenderableSeq for SeqRenderable<T, R>
where
    T: RenderableSeq,
    R: Renderable,
{
    #[tracing::instrument(skip(buf))]
    fn render_initial(self, area: Rect, buf: &mut Buffer) -> Rect {
        let area = self.init.render_initial(area, buf);
        self.r.into_widget().render(area, buf);
        area
    }
}

impl<T, R> Renderable for SeqRenderable<T, R>
where
    T: RenderableSeq,
    R: Renderable,
{
    fn into_widget(self) -> impl Widget {
        self
    }
}

impl<T, R> Widget for SeqRenderable<T, R>
where
    T: RenderableSeq,
    R: Renderable,
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = self.init.render_initial(area, buf);
        self.r.into_widget().render(area, buf);
    }
}

impl<T, R> DebugRollup for SeqRenderable<T, R>
where
    T: RenderableSeq,
    R: Renderable,
{
    fn rollup_entries(&self) -> debug_rollup::Entries<'_> {
        self.init.rollup_entries().with(&self.r)
    }
}
