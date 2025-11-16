use crossterm::event::Event;
use derive_debug::Dbg;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Style, Stylize as _};
use ratatui::widgets::Widget;
use ratatui_rseq::Renderable;

use crate::handler::Handler;

type Inner = tui_textarea::TextArea<'static>;

#[derive(Dbg)]
pub(crate) struct TextArea(#[dbg(formatter = "fmt_text_area")] Inner);

fn fmt_text_area(ta: &Inner) -> String {
    format!("{:?}", ta.lines())
}

impl TextArea {
    /// The height of the CommandInput
    pub(crate) fn height(&self) -> usize {
        self.0.lines().len()
    }

    pub(crate) fn insert_newline(&mut self) {
        self.0.insert_newline();
    }
}

impl Default for TextArea {
    fn default() -> Self {
        let mut inner = Inner::default();
        inner.set_cursor_style(Style::reset().on_light_cyan());
        inner.set_cursor_line_style(Style::default());
        inner.set_style(Style::reset().gray().on_dark_gray());
        TextArea(inner)
    }
}

impl Renderable for &TextArea {
    fn into_widget(self) -> impl Widget {
        &self.0
    }
}

impl Handler<Event> for TextArea {
    type Response = ();

    async fn handle(&mut self, msg: Event) -> Self::Response {
        self.0.input(msg);
    }
}
