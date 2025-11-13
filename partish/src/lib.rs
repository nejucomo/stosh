//! `partish` is a library and associated binary that runs an interactive "command-multiplexed" shell
//!
//! The term "command-multiplexed" means each command run in the user's shell has its own interactive ui widgets, which makes things like comparing two command runs or scrolling/collapsing the output of commands conveniently dynamic.
//!
//! This is different from a "terminal multiplexer" because there is only a single shell, and in fact, the author uses `partish` inside of a terminal multiplexer.
#![deny(unsafe_code, missing_docs)]
mod cmdinput;
mod handler;
mod mainscreen;
mod run;
mod u16util;
mod ui;

pub use self::run::run;
