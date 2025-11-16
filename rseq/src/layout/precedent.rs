use std::rc::Rc;

use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::widgets::Widget;

use crate::Renderable;
use crate::layout::{Constrained, Planner};

pub trait Precedent: std::fmt::Debug {
    fn map_layout<F>(self, f: F) -> Self
    where
        F: FnOnce(Layout) -> Layout;

    fn render_plan(
        self,
        loglabel: &'static str,
        constraints: Vec<Constraint>,
        area: Rect,
        buf: &mut Buffer,
    ) -> Rc<[Rect]>;

    fn dyn_debugs(&self) -> Vec<&dyn std::fmt::Debug>;
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
        caller_loglabel: &'static str,
        mut constraints: Vec<Constraint>,
        area: Rect,
        _: &mut Buffer,
    ) -> Rc<[Rect]> {
        constraints.reverse();
        let areas = self.constraints(constraints.clone()).split(area);
        if !area.is_empty() && areas.iter().any(|a| a.is_empty()) {
            tracing::warn!(
                ?caller_loglabel,
                ?area,
                ?constraints,
                ?areas,
                "empty area in layout of non-empty area!!!"
            );
        }
        areas
    }

    fn dyn_debugs(&self) -> Vec<&dyn std::fmt::Debug> {
        vec![]
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
            loglabel: self.loglabel,
            precedent: self.precedent.map_layout(f),
            subsequent: self.subsequent,
        }
    }

    #[tracing::instrument(skip(buf))]
    fn render_plan(
        self,
        caller_loglabel: &'static str,
        mut constraints: Vec<Constraint>,
        area: Rect,
        buf: &mut Buffer,
    ) -> Rc<[Rect]> {
        let Planner {
            loglabel: my_loglabel,
            precedent,
            subsequent: Constrained { constraint, r },
        } = self;

        let revix = constraints.len();
        constraints.push(constraint);

        let areas = precedent.render_plan(caller_loglabel, constraints, area, buf);

        let areacnt = areas.len();
        let ix = areacnt - 1 - revix;
        let render_area = areas[ix];

        if !render_area.is_empty() {
            r.into_widget().render(render_area, buf);
        } else {
            tracing::warn!(
                ?my_loglabel,
                ?areacnt,
                ?ix,
                ?render_area,
                "my render area is empty"
            );
        }
        areas
    }

    fn dyn_debugs(&self) -> Vec<&dyn std::fmt::Debug> {
        let mut v = self.precedent.dyn_debugs();
        v.push(&self.subsequent);
        v
    }
}
