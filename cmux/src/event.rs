use tokio_process_stream::Item;

use crate::Handle;

/// The items from the multiplexed stream of child events
#[derive(Debug)]
pub struct ChildEvent {
    /// The child's [Handle]
    pub handle: Handle,
    /// Input from the child
    pub item: Item<String>,
}
