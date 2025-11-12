//! [Renderable] builders to ergonomically produce [ratatui] widgets
#![deny(unsafe_code, missing_docs)]

pub mod layout;
mod renderable;
mod renderseq;
mod term;
mod widgets;

pub use self::renderable::Renderable;
pub use self::renderseq::{RenderableSeq, SeqRenderable};
pub use self::term::TerminalSession;
