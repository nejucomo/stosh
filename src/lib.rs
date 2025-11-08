//! `partish` is a library and associated binary that runs an interactive "command-multiplexed" shell
//!
//! The term "command-multiplexed" means each command run in the user's shell has its own interactive ui widgets, which makes things like comparing two command runs or scrolling/collapsing the output of commands conveniently dynamic.
//!
//! This is different from a "terminal multiplexer" because there is only a single shell, and in fact, the author uses `partish` inside of a terminal multiplexer.
#![deny(unsafe_code, missing_docs)]
mod display;
mod gadget;
pub mod gadgets;
mod handler;
mod notification;
mod rectext;
mod run;
mod ui;

pub use self::display::Display;
pub use self::gadget::Gadget;
pub use self::handler::EventHandler;
pub use self::notification::Notification;
pub use self::rectext::RectExt;
pub use self::run::run;
pub use self::ui::UI;
