use crossterm::event::Event;
use derive_debug::Dbg;
use derive_more::{Deref, DerefMut};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Style, Stylize as _};
use ratatui::widgets::Widget;
use ratatui_rseq::Renderable;

use crate::handler::Handler;

type Inner = tui_textarea::TextArea<'static>;

#[derive(Default, Dbg, Deref, DerefMut)]
pub(crate) struct TextArea(#[dbg(formatter = "fmt_text_area")] Inner);

fn fmt_text_area(ta: &Inner) -> String {
    format!("{:?}", ta.lines())
}

impl TextArea {
    /// The height of the CommandInput
    pub(crate) fn height(&self) -> usize {
        self.0.lines().len()
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
