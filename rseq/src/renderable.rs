use ratatui::layout::Constraint;
use ratatui::widgets::Widget;

use crate::layout;

/// A [Renderable] can convert to a [Widget] and provies composition methods
pub trait Renderable: Sized {
    /// Convert into a [Widget]
    fn into_widget(self) -> impl Widget;

    /// Constrain `self` for layout within a container
    fn constrained(self, constraint: Constraint) -> layout::Constrained<Self> {
        layout::Constrained::new(constraint, self)
    }
}

// Only a subset of widgets are directly [Renderable]:
impl<'a> Renderable for ratatui::text::Line<'a> {
    fn into_widget(self) -> impl Widget {
        self
    }
}

impl<'a> Renderable for ratatui::text::Text<'a> {
    fn into_widget(self) -> impl Widget {
        self
    }
}

impl<'a> Renderable for &tui_textarea::TextArea<'a> {
    fn into_widget(self) -> impl Widget {
        self
    }
}
