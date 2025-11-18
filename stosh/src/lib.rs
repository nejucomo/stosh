//! `stosh` - STack Of Shells
//!
//! This is a library and associated binary that provides a stack of shells.
//!
//! Each command run in the user's shell has its own interactive ui widgets, which makes things like comparing two command runs or scrolling/collapsing the output of commands conveniently dynamic.
#![deny(unsafe_code, missing_docs)]
mod cli;
mod cmd;
mod handler;
mod log;
mod mainscreen;
mod prompt;
mod rectext;
mod run;
mod u16util;
mod ui;

pub use self::run::run;
