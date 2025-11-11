use derive_new::new;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Rect};
use ratatui::text::Text;
use ratatui::widgets::{Block, Clear, Widget};

use crate::RectExt as _;

/// The stateful rendering context
#[derive(Debug, new)]
pub struct RenderContext<'b> {
    area: Rect,
    buf: &'b mut Buffer,
}

impl<'b> RenderContext<'b> {
    /// Render a [ContextualWidget]
    pub fn render<C>(&mut self, contextual: C) -> &mut Self
    where
        C: ContextualWidget,
    {
        contextual.render_to_context(self);
        self
    }

    fn render_widget<W>(&mut self, widget: W)
    where
        W: Widget,
    {
        widget.render(self.area, self.buf);
    }

    /// Convert a [ContextualWidget] into a [Widget]
    pub fn wrap_contextual<C>(contextual: C) -> impl Widget
    where
        C: ContextualWidget,
    {
        RenderContextWrapper(contextual)
    }
}

/// A tui element which renders to a [RenderContext]
pub trait ContextualWidget {
    /// Render into the context
    fn render_to_context<'b>(self, ctx: &mut RenderContext<'b>);
}

/// A [ContextualWidget] which simply centers the context within the given constraints
#[derive(Debug, new)]
pub struct CenteredOverlay {
    /// The horizontal constraint
    pub horizontal: Constraint,
    /// The vertical constraint
    pub vertical: Constraint,
}

impl ContextualWidget for CenteredOverlay {
    fn render_to_context<'b>(self, ctx: &mut RenderContext<'b>) {
        ctx.area = ctx.area.centered_subrect(self.horizontal, self.vertical);
    }
}

impl<T> ContextualWidget for Option<T>
where
    T: ContextualWidget,
{
    fn render_to_context<'b>(self, ctx: &mut RenderContext<'b>) {
        if let Some(t) = self {
            t.render_to_context(ctx);
        }
    }
}

impl<T> ContextualWidget for &Option<T>
where
    for<'t> &'t T: ContextualWidget,
{
    fn render_to_context<'b>(self, ctx: &mut RenderContext<'b>) {
        self.as_ref().render_to_context(ctx);
    }
}

impl ContextualWidget for Clear {
    fn render_to_context<'b>(self, ctx: &mut RenderContext<'b>) {
        ctx.render_widget(self);
    }
}

impl<'s> ContextualWidget for Block<'s> {
    fn render_to_context<'b>(self, ctx: &mut RenderContext<'b>) {
        let inner = self.inner(ctx.area);
        ctx.render_widget(self);
        ctx.area = inner;
    }
}

impl<'s> ContextualWidget for Text<'s> {
    fn render_to_context<'b>(self, ctx: &mut RenderContext<'b>) {
        ctx.render_widget(self);
    }
}

pub struct RenderContextWrapper<C>(C);

impl<C> Widget for RenderContextWrapper<C>
where
    C: ContextualWidget,
{
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        self.0.render_to_context(&mut RenderContext::new(area, buf))
    }
}
