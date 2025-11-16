use derive_new::new;
use ratatui::layout::Constraint::{Fill, Length};
use ratatui::style::{Style, Stylize as _};
use ratatui::text::Line;
use ratatui::widgets::Widget;
use ratatui_rseq::Renderable;

use crate::cmd;
use crate::u16util::IntoU16 as _;

/// A command portal allows viewing details about a command
#[derive(Debug)]
pub(crate) struct Portal {
    histix: usize,
    input: cmd::TextArea,
    output: cmd::TextArea,
}

impl Portal {
    pub(crate) fn new(histix: usize, mut input: cmd::TextArea) -> Self {
        let ds = Style::default();

        Portal {
            histix,
            input: input.reset_style().set_style(ds.gray().on_dark_gray()),
            output: cmd::TextArea::default().set_style(ds.blue().on_black()),
        }
    }

    // pub(crate) fn height(&self) -> usize {
    //     self.input.height() + 1
    // }
}

impl Renderable for &Portal {
    fn into_widget(self) -> impl Widget {
        let prompt = Line::from(format!("⟨{}⟩", self.histix).black().on_light_cyan());
        let pwidth = prompt.width().into_u16();

        prompt
            .constrained(Length(pwidth))
            .on_left("portal.prompt")
            .followed_by("portal.input", self.input.constrained(Fill(1)))
            .horizontal_margin(1)
            .spacing(1)
            .constrained(Length(1))
            .on_top("portal.histix-and-input")
            .followed_by("portal.output", self.output.constrained(Fill(1)))
    }
}
