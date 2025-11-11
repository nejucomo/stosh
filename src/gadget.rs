use crate::{ContextualWidget, EventHandler};

/// A widget that handles events
pub trait Gadget: EventHandler
where
    for<'s> &'s Self: ContextualWidget,
{
}
