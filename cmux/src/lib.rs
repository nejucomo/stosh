//! A [CommandMultiplexer] for interleaving subprocess I/O within a single task
#![deny(missing_docs)]
mod cmd;
mod cmux;
mod handle;
mod stream;

pub use self::cmd::Command;
pub use self::cmux::CommandMultiplexer;
pub use self::handle::Handle;
pub use self::stream::ProcessLineStream;
