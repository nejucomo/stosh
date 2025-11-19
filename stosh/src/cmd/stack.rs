use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::Widget;
use ratatui_rseq::Renderable;

use crate::cmd;
use crate::event::CommandEvent;
use crate::handler::Handler;
use crate::rectext::RectExt as _;
use crate::u16util::IntoU16 as _;

#[derive(Debug, Default)]
pub(crate) struct Stack {
    portals: Vec<cmd::Portal>,
}

impl Stack {
    pub(crate) fn push(&mut self, p: cmd::Portal) {
        self.portals.push(p);
    }
}

impl Handler<CommandEvent> for Stack {
    type Response = ();

    fn handle(&mut self, ev: CommandEvent) {
        self.portals[ev.handle].handle(ev.info);
    }
}

impl Renderable for &Stack {
    fn into_widget(self) -> impl Widget {
        self
    }
}

impl Widget for &Stack {
    #[tracing::instrument(skip(buf))]
    fn render(self, mut area: Rect, buf: &mut Buffer) {
        let ratio = (1, 3);

        for portal in self.portals.iter().rev() {
            if area.height == 0 {
                break;
            }

            let clipheight = area.height * ratio.1 / ratio.0;
            let clipheight = if clipheight == 0 {
                // Consume the remainder:
                area.height
            } else {
                clipheight
            };

            let splitheight = std::cmp::min(portal.height().into_u16(), clipheight);
            let (subarea, remaining) = area.split_vertically(splitheight);
            if area.is_empty() || subarea.is_empty() || remaining.is_empty() {
                tracing::warn!(?area.height, ?subarea.height, ?remaining.height, "empty areas in stack layout");
            }

            portal.into_widget().render(subarea, buf);
            area = remaining;
        }
    }
}
