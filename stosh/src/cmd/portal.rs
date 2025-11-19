use std::iter;
use std::process::ExitStatus;

use derive_new::new;
use ratatui::style::{Style, Styled as _, Stylize as _};
use ratatui::text::{Line, Text};
use ratatui::widgets::Widget;
use ratatui_rseq::Renderable;

use crate::event::CommandEventInfo;
use crate::handler::Handler;
use crate::{cmd, prompt};

use self::StreamSource::*;

/// A command portal allows viewing details about a command
#[derive(Debug, new)]
pub(crate) struct Portal {
    histix: cmd::Handle,
    input: Vec<String>,
    #[new(default)]
    output: Vec<(StreamSource, String)>,
    #[new(default)]
    outcome: Option<std::io::Result<ExitStatus>>,
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

impl Handler<CommandEventInfo> for Portal {
    type Response = ();

    fn handle(&mut self, ev: CommandEventInfo) {
        match ev {
            CommandEventInfo::Spawn(Ok(())) => {}
            CommandEventInfo::Spawn(Err(e)) => {
                self.outcome = Some(Err(e));
            }
            CommandEventInfo::Stdout(s) => {
                self.output.push((Stdout, s));
            }
            CommandEventInfo::Stderr(s) => {
                self.output.push((Stderr, s));
            }
            CommandEventInfo::Done(res) => {
                self.outcome = Some(res);
            }
        };
    }
}

impl Renderable for &Portal {
    fn into_widget(self) -> impl Widget {
        // Styles:
        let st_input = Style::default().gray().on_dark_gray();
        let st_text = Style::default().gray().on_black();
        let st_margin = match self.outcome.as_ref() {
            None => st_input,
            Some(Ok(status)) if status.success() => Style::default().gray().on_green(),
            Some(_) => Style::default().white().on_red(),
        };
        let st_out = st_margin.gray();
        let st_err = st_margin.yellow();

        let prompt = prompt::text(self.histix);
        let margin = prompt.replace(|_| true, " ");

        Some(prompt)
            .into_iter()
            .chain(iter::repeat(margin))
            .zip(self.input.iter())
            .map(|(mtext, text)| {
                [
                    mtext.set_style(st_margin),
                    text.as_str().set_style(st_input),
                ]
            })
            .chain(self.output.iter().map(|(src, text)| {
                [
                    match src {
                        Stdout => " • ".set_style(st_out),
                        Stderr => " ⚠️ ".set_style(st_err),
                    },
                    text.as_str().set_style(st_text),
                ]
            }))
            .chain(self.outcome.as_ref().map(|res| {
                [
                    " ◆ ".set_style(st_margin),
                    format!("{res:?}").set_style(st_margin),
                ]
            }))
            .map(Line::from_iter)
            .collect::<Text>()
    }
}
