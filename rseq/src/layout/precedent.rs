use std::rc::Rc;

use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::widgets::Widget;

use crate::Renderable;
use crate::layout::{Constrained, Planner};

pub trait Precedent {
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

    fn render_plan(
        self,
        mut constraints: Vec<Constraint>,
        area: Rect,
        _: &mut Buffer,
    ) -> Rc<[Rect]> {
        constraints.reverse();
        self.constraints(constraints).split(area)
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
            subsequent: self.subsequent,
        }
    }

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
        let ix = areas.len() - 1 - revix;
        r.into_widget().render(areas[ix], buf);
        areas
    }
}
