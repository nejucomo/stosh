mod seqr;

pub use self::seqr::SeqRenderable;

use debug_rollup::DebugRollup;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::Widget;

use crate::Renderable;

/// These are "builder" types which produce a [Renderable] with [RenderableSeq::then]
///
/// The impls are often containers or other building blocks for rendering.
pub trait RenderableSeq: Sized + DebugRollup {
    /// The impls handle special rendering here, returning a new context
    fn render_initial(self, area: Rect, buf: &mut Buffer) -> Rect;

    /// Application code uses this to convert `self` builder into a [Renderable]
    fn then<R>(self, r: R) -> SeqRenderable<Self, R>
    where
        R: Renderable,
    {
        SeqRenderable::new(self, r)
    }
}

impl<A, B> RenderableSeq for (A, B)
where
    A: RenderableSeq,
    B: RenderableSeq,
{
    fn render_initial(self, area: Rect, buf: &mut Buffer) -> Rect {
        let area = self.0.render_initial(area, buf);
        self.1.render_initial(area, buf)
    }
}

impl<A, B, C> RenderableSeq for (A, B, C)
where
    A: RenderableSeq,
    B: RenderableSeq,
    C: RenderableSeq,
{
    fn render_initial(self, area: Rect, buf: &mut Buffer) -> Rect {
        let (a, b, c) = self;
        ((a, b), c).render_initial(area, buf)
    }
}

impl RenderableSeq for ratatui::style::Style {
    fn render_initial(self, area: Rect, buf: &mut Buffer) -> Rect {
        buf.set_style(area, self);
        area
    }
}

impl RenderableSeq for ratatui::widgets::Clear {
    fn render_initial(self, area: Rect, buf: &mut Buffer) -> Rect {
        self.render(area, buf);
        area
    }
}

impl<'a> RenderableSeq for ratatui::widgets::Block<'a> {
    #[tracing::instrument(skip(self, buf))]
    fn render_initial(self, area: Rect, buf: &mut Buffer) -> Rect {
        let inner = self.inner(area);
        tracing::debug!(?area, ?inner);
        if inner.is_empty() {
            tracing::warn!(?inner, "empty rendering area");
        }
        self.render(area, buf);
        inner
    }
}
