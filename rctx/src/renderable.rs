use crate::RenderContext;

/// Similar to [ratatui::widgets::Widget]
pub trait Renderable: Sized {
    /// Render into `rctx`
    fn render_into<'b>(self, rctx: RenderContext<'b>);
}
