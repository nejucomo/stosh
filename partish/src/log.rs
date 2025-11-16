use std::fmt::Display;
use std::path::PathBuf;
use std::str::FromStr;

use derive_more::{AsRef, Deref};
use time::{OffsetDateTime, format_description::well_known::Rfc3339};

/// A [PathBuf] newtype with a sensible default for our logfile
#[derive(Clone, Debug, Deref, AsRef)]
#[as_ref(forward)]
pub struct LogPath(PathBuf);

impl Default for LogPath {
    fn default() -> Self {
        let now = OffsetDateTime::now_local().unwrap();
        let timestamp = now.format(&Rfc3339).unwrap();

        let mut p =
            dirs::data_local_dir().expect("`dirs::data_local_dir()` undefined on this platform");
        p.push(env!("CARGO_PKG_NAME"));
        p.push(format!("log_{timestamp}.txt"));
        LogPath(p)
    }
}

impl FromStr for LogPath {
    type Err = <PathBuf as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        PathBuf::from_str(s).map(LogPath)
    }
}

impl Display for LogPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display().fmt(f)
    }
}
