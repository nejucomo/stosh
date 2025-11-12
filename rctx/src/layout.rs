//! Layout builders
use derive_new::new;
use ratatui::layout::{Constraint, Direction, Flex, Layout, Spacing};

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

    /// Place `self` to the left of another widget
    pub fn on_top(self) -> Pending<R> {
        Pending::new(Direction::Vertical, self)
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
#[derive(Debug)]
pub struct LayoutPair<A, B>
where
    A: Renderable,
    B: Renderable,
{
    layout: Layout,
    a: Constrained<A>,
    b: Constrained<B>,
}

impl<A, B> LayoutPair<A, B>
where
    A: Renderable,
    B: Renderable,
{
    fn new(direction: Direction, a: Constrained<A>, b: Constrained<B>) -> Self {
        LayoutPair {
            layout: Layout::new(direction, [a.constraint, b.constraint]),
            a,
            b,
        }
    }

    fn map_layout<F>(self, f: F) -> Self
    where
        F: FnOnce(Layout) -> Layout,
    {
        LayoutPair {
            layout: f(self.layout),
            a: self.a,
            b: self.b,
        }
    }

    /// Set the margin; see [Layout::margin]
    pub fn margin(self, margin: u16) -> Self {
        self.map_layout(|l| l.margin(margin))
    }

    /// Set the horizontal margin; see [Layout::horizontal_margin]
    pub fn horizontal_margin(self, margin: u16) -> Self {
        self.map_layout(|l| l.horizontal_margin(margin))
    }

    /// Set the vertical margin; see [Layout::vertical_margin]
    pub fn vertical_margin(self, margin: u16) -> Self {
        self.map_layout(|l| l.vertical_margin(margin))
    }

    /// Set the flex; see [Layout::flex]
    pub fn flex(self, flex: Flex) -> Self {
        self.map_layout(|l| l.flex(flex))
    }

    /// Set the spacing; see [Layout::spacing]
    pub fn spacing<T>(self, spacing: T) -> Self
    where
        T: Into<Spacing>,
    {
        self.map_layout(|l| l.spacing(spacing))
    }
}

impl<A, B> Renderable for LayoutPair<A, B>
where
    A: Renderable,
    B: Renderable,
{
    fn render_into<'b>(self, rctx: RenderContext<'b>) {
        let LayoutPair { layout, a, b } = self;
        let RenderContext { area, buf } = rctx;
        let [area_a, area_b] = layout.areas(area);
        RenderContext::new(area_a, buf).render(a.r);
        RenderContext::new(area_b, buf).render(b.r);
    }
}
