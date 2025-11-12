use derive_new::new;
use ratatui::layout::{Constraint, Direction, Layout};

use crate::Renderable;
use crate::layout::Planner;

/// A constraint-associated renderable
#[derive(Debug, new)]
pub struct Constrained<R>
where
    R: Renderable,
{
    pub(super) constraint: Constraint,
    pub(super) r: R,
}

impl<R> Constrained<R>
where
    R: Renderable,
{
    /// Place `self` to the left of another widget
    pub fn on_left(self) -> Planner<Layout, R> {
        Planner::new_direction(Direction::Horizontal, self)
    }

    /// Place `self` to the left of another widget
    pub fn on_top(self) -> Planner<Layout, R> {
        Planner::new_direction(Direction::Vertical, self)
    }
}
