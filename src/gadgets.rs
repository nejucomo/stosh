//! A collection of [Gadget](crate::Gadget) types compromising the ui
mod exitdialog;
mod main;
mod root;

pub use self::exitdialog::ExitDialog;
pub use self::main::MainPane;
pub use self::root::RootGadget;
