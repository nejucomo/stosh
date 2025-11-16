use derive_new::new;
use ratatui::layout::Constraint::{Fill, Length};
use ratatui::style::{Style, Stylize as _};
use ratatui::text::Line;
use ratatui::widgets::Widget;
use ratatui_rseq::Renderable;

use crate::u16util::IntoU16 as _;
use crate::{cmd, prompt};

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
        let mut output = cmd::TextArea::default().set_style(ds.blue().on_black());
        output.insert_str("hello\nworld\nhere\nis\nsome\ntext\nover\nlines");

        Portal {
            histix,
            input: input.reset_style().set_style(ds.gray().on_dark_gray()),
            output,
        }
    }

    pub(crate) fn height(&self) -> usize {
        self.input.height() + self.output.height()
    }
}

impl Renderable for &Portal {
    fn into_widget(self) -> impl Widget {
        let prompt = Line::from(prompt::text(self.histix).black().on_cyan());
        let pwidth = prompt.width().into_u16();

        prompt
            .constrained(Length(pwidth))
            .on_left()
            .followed_by(
                self.input
                    .constrained(Length(1))
                    .on_top()
                    .followed_by(self.output.constrained(Fill(1)))
                    .constrained(Fill(1)),
            )
            .horizontal_margin(1)
            .spacing(1)
    }
}
