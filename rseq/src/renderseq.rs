use derive_new::new;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::Widget;

use crate::Renderable;

/// These are "builder" types which produce a [Renderable] with [RenderableSeq::then]
///
/// The impls are often containers or other building blocks for rendering.
pub trait RenderableSeq: Sized {
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

/// A [Renderable] which encapsulates a sequence produced by [RenderableSeq::then]
#[derive(Debug, new)]
pub struct SeqRenderable<T, R>
where
    T: RenderableSeq,
    R: Renderable,
{
    init: T,
    r: R,
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
    fn render_initial(self, area: Rect, buf: &mut Buffer) -> Rect {
        let inner = self.inner(area);
        self.render(area, buf);
        inner
    }
}
