use clap::Parser;

use crate::log::LogPath;

/// An interactive terminal command stack
#[derive(Parser)]
#[command(version, about)]
pub(crate) struct Options {
    /// The log file path
    #[arg(short, long, value_name = "PATH", default_value_t)]
    pub(crate) log_path: LogPath,
}

impl Options {
    pub(crate) fn parse() -> Self {
        <Self as Parser>::parse()
    }
}
