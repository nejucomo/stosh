use std::rc::Rc;

use debug_rollup::DebugRollup;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::widgets::Widget;

use crate::Renderable;
use crate::layout::{Constrained, Planner};

pub trait Precedent: DebugRollup {
    fn map_layout<F>(self, f: F) -> Self
    where
        F: FnOnce(Layout) -> Layout;

    fn render_plan(self, constraints: Vec<Constraint>, area: Rect, buf: &mut Buffer) -> Rc<[Rect]>;
}

impl Precedent for Layout {
    fn map_layout<F>(self, f: F) -> Self
    where
        F: FnOnce(Layout) -> Layout,
    {
        f(self)
    }

    #[tracing::instrument]
    fn render_plan(
        self,
        mut constraints: Vec<Constraint>,
        area: Rect,
        _: &mut Buffer,
    ) -> Rc<[Rect]> {
        constraints.reverse();
        let areas = self.constraints(constraints.clone()).split(area);
        if !area.is_empty() && areas.iter().any(|a| a.is_empty()) {
            tracing::warn!(
                ?area,
                ?constraints,
                ?areas,
                "empty area in layout of non-empty area!!!"
            );
        }
        areas
    }
}

impl<P, S> Precedent for Planner<P, S>
where
    P: Precedent,
    S: Renderable,
{
    fn map_layout<F>(self, f: F) -> Self
    where
        F: FnOnce(Layout) -> Layout,
    {
        Planner {
            precedent: self.precedent.map_layout(f),
            ..self
        }
    }

    #[tracing::instrument(skip(buf))]
    fn render_plan(
        self,
        mut constraints: Vec<Constraint>,
        area: Rect,
        buf: &mut Buffer,
    ) -> Rc<[Rect]> {
        let Planner {
            precedent,
            subsequent: Constrained { constraint, r },
        } = self;

        let revix = constraints.len();
        constraints.push(constraint);

        let areas = precedent.render_plan(constraints, area, buf);

        let areacnt = areas.len();
        let ix = areacnt - 1 - revix;
        let render_area = areas[ix];

        if render_area.is_empty() {
            tracing::warn!(?areacnt, ?ix, ?render_area, "empty render area");
        }
        r.into_widget().render(render_area, buf);
        areas
    }
}
