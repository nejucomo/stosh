//! A [CommandMultiplexer] for interleaving subprocess I/O within a single task
#![deny(missing_docs)]
mod cmux;
mod event;
mod stream;

pub use self::cmux::CommandMultiplexer;
pub use self::event::ChildEvent;
pub use self::stream::ProcessLineStream;
