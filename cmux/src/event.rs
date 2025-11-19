use tokio_process_stream::Item;

/// The items from the multiplexed stream of child events
#[derive(Debug)]
pub struct ChildEvent<T> {
    /// The child's associated userdata
    pub userdata: T,
    /// Input from the child
    pub item: Item<String>,
}
