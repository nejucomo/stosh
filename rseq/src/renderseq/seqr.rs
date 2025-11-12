use derive_new::new;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::Widget;

use crate::{Renderable, RenderableSeq};

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
