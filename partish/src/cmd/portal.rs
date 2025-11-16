use derive_new::new;
use ratatui::widgets::Widget;
use ratatui_rseq::Renderable;

use crate::cmd;

/// A command portal allows viewing details about a command
#[derive(Debug, new)]
pub(crate) struct Portal {
    histix: usize,
    text: cmd::TextArea,
}

impl Portal {
    // pub(crate) fn height(&self) -> usize {
    //     self.input.height() + 1
    // }
}

impl Renderable for &Portal {
    fn into_widget(self) -> impl Widget {
        self.text.into_widget()
    }
}
