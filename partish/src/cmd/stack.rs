use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint::Ratio, Layout, Rect};
use ratatui::style::{Style, Stylize as _};
use ratatui::widgets::Widget;
use ratatui_rseq::Renderable;

use crate::cmd;

#[derive(Debug, Default)]
pub(crate) struct Stack {
    portals: Vec<cmd::Portal>,
}

impl Stack {
    pub(crate) fn handle_new_input(&mut self, input: cmd::TextArea) -> std::io::Result<()> {
        self.portals.push(cmd::Portal::new(input));
        Ok(())
    }
}

impl Renderable for &Stack {
    fn into_widget(self) -> impl Widget {
        self
    }
}

impl Widget for &Stack {
    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, Style::reset().cyan().on_green());

        let areas =
            Layout::vertical(std::iter::repeat_n(Ratio(1, 3), self.portals.len())).split(area);

        for (i, portal) in self.portals.iter().enumerate() {
            portal.into_widget().render(areas[i], buf);
        }
    }
}
