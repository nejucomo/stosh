use std::os::unix::process::ExitStatusExt;
use std::process::ExitStatus;

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Style, Styled as _};
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, Padding, Widget};
use ratatui_rseq::{Renderable, RenderableSeq as _};

use crate::cuteblock::CuteBlock;
use crate::event::CommandEventInfo;
use crate::handler::Handler;
use crate::styles::STYLES;
use crate::{cmd, prompt};

use self::StreamSource::*;

/// A command portal allows viewing details about a command
#[derive(Debug)]
pub(crate) struct Portal {
    histix: cmd::Handle,
    input: Vec<String>,
    output: Vec<(StreamSource, String)>,
    outcome: Option<std::io::Result<ExitStatus>>,
}

#[derive(Debug)]
enum StreamSource {
    Stdout,
    Stderr,
}

impl Portal {
    pub(crate) fn new(histix: cmd::Handle, input: Vec<String>) -> Self {
        let input: Vec<String> = input
            .into_iter()
            .filter(|line| !line.trim().is_empty())
            .collect();

        assert!(!input.is_empty());
        Portal {
            histix,
            input,
            output: vec![],
            outcome: None,
        }
    }

    pub(crate) fn height(&self) -> usize {
        let borders = 2;
        borders + self.input.len() + self.output.len()
    }

    pub(crate) fn cmd_name(&self) -> &str {
        self.input
            .iter()
            .flat_map(|l| l.split(' '))
            .find(|w| !w.trim().is_empty())
            .expect("no cmd_name: empty input")
    }

    fn get_cute_block(&self) -> CuteBlock<'_> {
        let (status_style, outcome_text) = self.describe_outcome();

        let cb = CuteBlock::bordered()
            .title_top(prompt::text(self.histix).set_style(STYLES.text.histix))
            .title_top(self.cmd_name().set_style(status_style))
            .border_style(STYLES.border.view.style)
            .border_type(STYLES.border.view.btype)
            .padding(Padding::horizontal(1));

        if let Some(ot) = outcome_text {
            cb.title_top(Line::from(ot.set_style(status_style)).right_aligned())
        } else {
            cb
        }
    }

    fn describe_outcome(&self) -> (Style, Option<String>) {
        match self.outcome.as_ref() {
            Some(Ok(status)) => {
                if status.success() {
                    (STYLES.status.exit_success, Some("✓".to_string()))
                } else if status.continued() {
                    (STYLES.status.running, Some("⏱️".to_string()))
                } else if let Some(ss) = status.stopped_signal() {
                    (STYLES.status.spawning, Some(format!("stopped: {ss}")))
                } else {
                    (
                        STYLES.status.exit_error,
                        Some(
                            [
                                if status.core_dumped() {
                                    Some("☢️".to_string())
                                } else {
                                    None
                                },
                                status.code().map(|code| format!("exit: {code}")),
                                status.signal().map(|sig| format!("signal: {sig}")),
                            ]
                            .into_iter()
                            .flatten() // Filter for `Some` values
                            .intersperse(" | ".to_string())
                            .collect(),
                        ),
                    )
                }
            }
            Some(Err(e)) => (STYLES.status.spawn_err, Some(format!("Error: {e}"))),
            None => (STYLES.status.running, None),
        }
    }

    fn render_heights<T>(&self, height_inner: T) -> (usize, usize)
    where
        T: Into<usize>,
    {
        let h = height_inner.into();
        let outlen = self.output.len();
        let hinput = self.input.len().clamp(1, h - h.min(outlen));
        let houtput = outlen.clamp(0, hinput - hinput.min(outlen));
        (hinput, houtput)
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
        self
    }
}

impl Widget for &Portal {
    fn render(self, area_outer: Rect, buf: &mut Buffer) {
        let block = Block::from(self.get_cute_block());
        let area = block.inner(area_outer);
        let (hinput, houtput) = self.render_heights(area.height);
        let out_skip = self.output.len() - houtput;

        block
            .then(Text::from_iter(
                self.input
                    .iter()
                    .take(hinput)
                    .cloned()
                    .map(|line| line.set_style(STYLES.text.input))
                    .chain(self.output.iter().skip(out_skip).map(|(src, line)| {
                        line.to_string().set_style(match src {
                            Stdout => STYLES.text.stdout,
                            Stderr => STYLES.text.stderr,
                        })
                    })),
            ))
            .into_widget()
            .render(area, buf);
    }
}
