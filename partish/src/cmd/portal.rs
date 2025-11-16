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
        input.set_cursor_style(Style::default());
        input.set_cursor_line_style(Style::default());
        input.set_style(Style::default().gray().on_dark_gray());

        let mut output = cmd::TextArea::default();
        input.set_style(Style::reset().blue().on_black());

        Portal {
            histix,
            input,
            output,
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
