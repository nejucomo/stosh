//! Various supporting [Widget] types

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::Widget;

#[derive(Debug)]
pub struct OptionalWidget<W>(pub(crate) Option<W>);

impl<W> Widget for OptionalWidget<W>
where
    W: Widget,
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        if let Some(w) = self.0 {
            w.render(area, buf);
        }
    }
}
