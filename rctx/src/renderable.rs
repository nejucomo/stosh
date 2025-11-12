use crate::{FilledBlock, RenderContext};

/// Similar to [ratatui::widgets::Widget]
pub trait Renderable: Sized {
    /// Render into `rctx`
    fn render_into<'b>(self, rctx: RenderContext<'b>);

    /// Place `self` visually inside a [FilledBlock]
    fn within_block<'b>(self) -> FilledBlock<'b, Self> {
        FilledBlock::new(self)
    }
}
