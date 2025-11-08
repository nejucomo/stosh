use ratatui::widgets::WidgetRef;

use crate::EventHandler;

/// A widget that handles events
pub trait Gadget: WidgetRef + EventHandler {}
