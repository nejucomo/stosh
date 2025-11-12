use derive_new::new;
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Widget as _};

use crate::{RenderContext, Renderable};

/// A block with a [Renderable] inside
#[derive(Debug, new)]
pub struct FilledBlock<'a, R>
where
    R: Renderable,
{
    #[new(default)]
    block: Block<'a>,
    inner: R,
}

impl<'a, R> FilledBlock<'a, R>
where
    R: Renderable,
{
    /// Add a title similar to [Block::title_top]
    pub fn title_top<T>(self, title: T) -> Self
    where
        T: Into<Line<'a>>,
    {
        self.map_block(|b| b.title_top(title))
    }

    /// Add a title similar to [Block::title_bottom]
    pub fn title_bottom<T>(self, title: T) -> Self
    where
        T: Into<Line<'a>>,
    {
        self.map_block(|b| b.title_bottom(title))
    }

    /// Define which borders to display similar to [Block::borders]
    pub fn borders(self, flag: Borders) -> Self {
        self.map_block(|b| b.borders(flag))
    }

    /// Define  border style similar to [Block::borders]
    pub fn border_style<S>(self, style: S) -> Self
    where
        S: Into<Style>,
    {
        self.map_block(|b| b.border_style(style))
    }

    fn map_block<F>(self, f: F) -> Self
    where
        F: FnOnce(Block<'a>) -> Block<'a>,
    {
        FilledBlock {
            block: f(self.block),
            inner: self.inner,
        }
    }
}

impl<'s, R> Renderable for FilledBlock<'s, R>
where
    R: Renderable,
{
    fn render_into<'b>(self, rctx: RenderContext<'b>) {
        let FilledBlock { block, inner } = self;
        let RenderContext { area, buf } = rctx;
        let innerarea = block.inner(area);

        block.render(area, buf);
        RenderContext::new(innerarea, buf).render(inner);
    }
}
