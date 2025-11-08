use ratatui::layout::{Constraint, Flex, Layout, Rect};

/// An extension trait for [Rect]
pub trait RectExt: Sized {
    /// Return a centered [Rect] within `self` matching the constraints
    fn centered_subrect(self, horizontal: Constraint, vertical: Constraint) -> Rect {
        self.centered_horizontal(horizontal)
            .centered_vertical(vertical)
    }

    /// Return a horizontally centered [Rect] within `self` matching the constraint
    fn centered_horizontal(self, cons: Constraint) -> Rect;

    /// Return a vertically centered [Rect] within `self` matching the constraint
    fn centered_vertical(self, cons: Constraint) -> Rect;
}

impl RectExt for Rect {
    fn centered_horizontal(self, cons: Constraint) -> Rect {
        let [area] = Layout::horizontal([cons]).flex(Flex::Center).areas(self);
        area
    }

    fn centered_vertical(self, cons: Constraint) -> Rect {
        let [area] = Layout::vertical([cons]).flex(Flex::Center).areas(self);
        area
    }
}
