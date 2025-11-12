//! Layout builders
use derive_new::new;
use ratatui::layout::{Constraint, Direction, Layout};

use crate::{RenderContext, Renderable};

/// A constraint-associated renderable
#[derive(Debug, new)]
pub struct Constrained<R>
where
    R: Renderable,
{
    constraint: Constraint,
    r: R,
}

impl<R> Constrained<R>
where
    R: Renderable,
{
    /// Place `self` to the left of another widget
    pub fn on_left(self) -> Pending<R> {
        Pending::new(Direction::Horizontal, self)
    }
}

/// An incomplete layout with a single child element
#[derive(Debug, new)]
pub struct Pending<A>
where
    A: Renderable,
{
    direction: Direction,
    a: Constrained<A>,
}

impl<A> Pending<A>
where
    A: Renderable,
{
    /// Complete a laid-out pair
    pub fn followed_by<B>(self, subsequent: Constrained<B>) -> LayoutPair<A, B>
    where
        B: Renderable,
    {
        LayoutPair::new(self.direction, self.a, subsequent)
    }
}

/// A complete laid out pair
#[derive(Debug, new)]
pub struct LayoutPair<A, B>
where
    A: Renderable,
    B: Renderable,
{
    direction: Direction,
    a: Constrained<A>,
    b: Constrained<B>,
}

impl<A, B> Renderable for LayoutPair<A, B>
where
    A: Renderable,
    B: Renderable,
{
    fn render_into<'b>(self, rctx: RenderContext<'b>) {
        let LayoutPair { direction, a, b } = self;
        let RenderContext { area, buf } = rctx;
        let layout = Layout::new(direction, [a.constraint, b.constraint]);
        let [area_a, area_b] = layout.areas(area);
        RenderContext::new(area_a, buf).render(a.r);
        RenderContext::new(area_b, buf).render(b.r);
    }
}
