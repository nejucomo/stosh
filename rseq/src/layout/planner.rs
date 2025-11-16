#[path = "precedent.rs"]
mod sealed;

use debug_rollup::{DebugRollup, delegate_debug_to_rollup};
use derive_new::new;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Direction, Flex, Layout, Rect, Spacing};
use ratatui::widgets::Widget;

use crate::Renderable;
use crate::layout::Constrained;
use crate::layout::planner::sealed::Precedent as _;

/// A one-dimensional layout planner
#[derive(new)]
#[new(visibility = "")]
pub struct Planner<P, S>
where
    P: sealed::Precedent,
    S: Renderable,
{
    loglabel: &'static str,
    precedent: P,
    subsequent: Constrained<S>,
}

impl<S> Planner<Layout, S>
where
    S: Renderable,
{
    pub(super) fn new_direction(
        loglabel: &'static str,
        d: Direction,
        subsequent: Constrained<S>,
    ) -> Self {
        let c: [Constraint; 0] = [];
        let layout = Layout::new(d, c);
        Planner::new(loglabel, layout, subsequent)
    }
}

impl<P, S> Planner<P, S>
where
    P: sealed::Precedent,
    S: Renderable,
{
    /// Append another element
    pub fn followed_by<R>(self, loglabel: &'static str, r: Constrained<R>) -> Planner<Self, R>
    where
        R: Renderable,
    {
        Planner::new(loglabel, self, r)
    }

    /// Adjust the margin as per [Layout::margin]
    pub fn margin(self, margin: u16) -> Self {
        self.map_layout(|l| l.margin(margin))
    }

    /// Adjust the horizontal_margin as per [Layout::horizontal_margin]
    pub fn horizontal_margin(self, horizontal: u16) -> Self {
        self.map_layout(|l| l.horizontal_margin(horizontal))
    }

    /// Adjust the vertical_margin as per [Layout::vertical_margin]
    pub fn vertical_margin(self, vertical: u16) -> Self {
        self.map_layout(|l| l.vertical_margin(vertical))
    }

    /// Adjust the flex as per [Layout::flex]
    pub fn flex(self, flex: Flex) -> Self {
        self.map_layout(|l| l.flex(flex))
    }

    /// Adjust the spacing as per [Layout::spacing]
    pub fn spacing<T>(self, spacing: T) -> Self
    where
        T: Into<Spacing>,
    {
        self.map_layout(|l| l.spacing(spacing))
    }
}

impl<P, S> Renderable for Planner<P, S>
where
    P: sealed::Precedent,
    S: Renderable,
{
    fn into_widget(self) -> impl Widget {
        self
    }
}

impl<P, S> Widget for Planner<P, S>
where
    P: sealed::Precedent,
    S: Renderable,
{
    #[tracing::instrument(skip(buf))]
    fn render(self, area: Rect, buf: &mut Buffer) {
        let loglabel = self.loglabel;
        if area.is_empty() {
            tracing::warn!(?loglabel, "empty area detected");
        }
        self.render_plan(loglabel, vec![], area, buf);
    }
}

impl<P, S> DebugRollup for Planner<P, S>
where
    P: sealed::Precedent,
    S: Renderable,
{
    fn dyn_debugs(&self) -> Vec<Box<dyn std::fmt::Debug + '_>> {
        let mut v = self.precedent.dyn_debugs();
        v.push(Box::new(&self.subsequent));
        v
    }
}

delegate_debug_to_rollup!(
    Planner<P, S>
    where
        P: sealed::Precedent,
        S: Renderable,
);
