use derive_new::new;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::Widget as _;

use crate::Renderable;

/// Encapsulate the rendering target area and buffer from [ratatui]
#[derive(Debug, new)]
pub struct RenderContext<'b> {
    area: Rect,
    buf: &'b mut Buffer,
}

impl<'t> Renderable for ratatui::text::Text<'t> {
    fn render_into<'b>(self, rctx: RenderContext<'b>) {
        self.render(rctx.area, rctx.buf);
    }
}
