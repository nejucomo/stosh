use ratatui::layout::Constraint;

use crate::{FilledBlock, RenderContext, layout};

/// Similar to [ratatui::widgets::Widget]
pub trait Renderable: Sized {
    /// Render into `rctx`
    fn render_into<'b>(self, rctx: RenderContext<'b>);

    /// Place `self` visually inside a [FilledBlock]
    fn within_block<'b>(self) -> FilledBlock<'b, Self> {
        FilledBlock::new(self)
    }

    /// Constrain `self` for layout within a container
    fn constrained(self, constraint: Constraint) -> layout::Constrained<Self> {
        layout::Constrained::new(constraint, self)
    }
}

impl<'a> Renderable for ratatui::text::Line<'a> {
    fn render_into<'b>(self, rctx: RenderContext<'b>) {
        rctx.render_widget(self);
    }
}

impl<'a> Renderable for ratatui::text::Text<'a> {
    fn render_into<'b>(self, rctx: RenderContext<'b>) {
        rctx.render_widget(self);
    }
}

impl<'a> Renderable for &tui_textarea::TextArea<'a> {
    fn render_into<'b>(self, rctx: RenderContext<'b>) {
        rctx.render_widget(self);
    }
}
