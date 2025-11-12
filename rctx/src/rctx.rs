use derive_new::new;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::Widget;

use crate::Renderable;

/// Encapsulate the rendering target area and buffer from [ratatui]
#[derive(Debug, new)]
pub struct RenderContext<'b> {
    pub(crate) area: Rect,
    pub(crate) buf: &'b mut Buffer,
}

impl<'b> RenderContext<'b> {
    /// Render into `self`
    pub fn render<R>(self, r: R)
    where
        R: Renderable,
    {
        r.render_into(self);
    }

    /// Render a [Widget] which is crate private to prevent "incomplete container" renders
    #[allow(dead_code)]
    pub(crate) fn render_widget<W>(&mut self, widget: W)
    where
        W: Widget,
    {
        widget.render(self.area, self.buf);
    }
}

impl<'t> Renderable for ratatui::text::Text<'t> {
    fn render_into<'b>(self, rctx: RenderContext<'b>) {
        self.render(rctx.area, rctx.buf);
    }
}
