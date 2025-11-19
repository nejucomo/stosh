use crossterm::event::Event;
use derive_debug::Dbg;
use derive_more::{Deref, DerefMut};
use ratatui::style::Style;
use ratatui::widgets::Widget;
use ratatui_rseq::Renderable;

use crate::handler::Handler;

type Inner = tui_textarea::TextArea<'static>;

#[derive(Dbg, Deref, DerefMut)]
pub(crate) struct TextArea(#[dbg(formatter = "fmt_text_area")] Inner);

fn fmt_text_area(ta: &Inner) -> String {
    format!("{:?}", ta.lines())
}

impl TextArea {
    pub(crate) fn reset_style(self) -> Self {
        let s = Style::default();

        self.set_cursor_style(s)
            .set_cursor_line_style(s)
            .set_style(s)
    }

    pub(crate) fn set_style(mut self, style: Style) -> Self {
        self.0.set_style(style);
        self
    }

    pub(crate) fn set_cursor_style(mut self, style: Style) -> Self {
        self.0.set_cursor_style(style);
        self
    }

    pub(crate) fn set_cursor_line_style(mut self, style: Style) -> Self {
        self.0.set_cursor_line_style(style);
        self
    }

    /// The height of the CommandInput
    pub(crate) fn height(&self) -> usize {
        self.0.lines().len()
    }
}

impl Default for TextArea {
    fn default() -> Self {
        TextArea(Inner::default()).reset_style()
    }
}

impl Renderable for &TextArea {
    fn into_widget(self) -> impl Widget {
        &self.0
    }
}

impl Handler<Event> for TextArea {
    type Response = ();

    fn handle(&mut self, ev: Event) -> std::io::Result<Self::Response> {
        self.0.input(ev);
        Ok(())
    }
}
