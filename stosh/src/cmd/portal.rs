mod mode;

use derive_new::new;
use ratatui::layout::Constraint::{Fill, Length};
use ratatui::style::{Style, Stylize as _};
use ratatui::text::Line;
use ratatui::widgets::Widget;
use ratatui_rseq::Renderable;

use crate::event::CommandEventInfo;
use crate::handler::Handler;
use crate::u16util::IntoU16 as _;
use crate::{cmd, prompt};

/// A command portal allows viewing details about a command
#[derive(Debug, new)]
pub(crate) struct Portal {
    histix: cmd::Handle,
    #[new(default)]
    input: Vec<String>,
    #[new(default)]
    output: Vec<(StreamSource, String)>,
    #[new(default)]
    outcome: Option<std::io::Result<()>>,
}

#[derive(Debug)]
enum StreamSource {
    Stdout,
    Stderr,
}

impl Portal {
    pub(crate) fn height(&self) -> usize {
        self.input.len() + self.output.len() + if self.outcome.is_some() { 1 } else { 0 }
    }
}

impl Renderable for &Portal {
    fn into_widget(self) -> impl Widget {
        let ds = Style::default();
        let mut output = cmd::TextArea::default().set_style(ds.blue().on_black());
        output.insert_str("hello\nworld\nhere\nis\nsome\ntext\nover\nlines");

        let input = cmd::TextArea::default().set_style(ds.gray().on_dark_gray());

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

impl Handler<CommandEventInfo> for Portal {
    type Response = ();

    fn handle(&mut self, ev: CommandEventInfo) -> std::io::Result<()> {
        use CommandEvent::Info;

        let (newstate, msg) = match ev {
            CommandEventInfo::Spawn(_) => todo!(),
            CommandEventInfo::Stdout(_) => todo!(),
            CommandEventInfo::Stderr(_) => todo!(),
            CommandEventInfo::Done(exit_status) => todo!(),
        };
        Ok(())
    }
}
