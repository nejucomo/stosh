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
        let st_status = match self.outcome.as_ref() {
            None => st_input,
            Some(Ok(status)) if status.success() => Style::default().gray().on_green(),
            Some(_) => Style::default().white().on_red(),
        };
        let st_out = st_text;
        let st_err = st_text.yellow();

        let prompt = prompt::text(self.histix);
        let margin = prompt.replace(|_| true, " ");
        let pad = " ".set_style(st_text);

        Some(prompt)
            .into_iter()
            .chain(iter::repeat(margin.clone()))
            .zip(self.input.iter())
            .map(|(mtext, text)| {
                [
                    pad.clone(),
                    mtext.set_style(st_status),
                    pad.clone(),
                    text.as_str().set_style(st_input),
                ]
            })
            .chain(self.output.iter().map(|(src, text)| {
                let (m, st) = match src {
                    Stdout => (margin.clone(), st_out),
                    Stderr => (" ⚠️ ".to_string(), st_err),
                };

                [
                    pad.clone(),
                    m.set_style(st),
                    pad.clone(),
                    text.as_str().set_style(st),
                ]
            }))
            .chain(self.outcome.as_ref().map(|res| {
                [
                    pad.clone(),
                    " ◆ ".set_style(st_status),
                    pad.clone(),
                    format!("{res:?}").set_style(st_text),
                ]
            }))
            .map(Line::from_iter)
            .collect::<Text>()
    }
}
