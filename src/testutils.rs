//! test utilities
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;

/// Extend [Buffer] with a convenient parser
pub trait BufferExt: Sized {
    /// Parse rows
    fn parse(text: &str) -> std::io::Result<Self>;
}

impl BufferExt for Buffer {
    fn parse(text: &str) -> std::io::Result<Self> {
        let usize_to_u16 = |u| u16::try_from(u).map_err(std::io::Error::other);

        let mut buf = Buffer::empty(Rect::new(
            0,
            0,
            usize_to_u16(
                text.lines()
                    .map(|s| s.chars().count() - 2)
                    .max()
                    .unwrap_or_default(),
            )?,
            usize_to_u16(text.lines().count())?,
        ));

        for (row, line) in text.lines().enumerate() {
            let line = line
                .strip_prefix("|")
                .ok_or_else(|| std::io::Error::other("missing '|' prefix delimiter"))?
                .strip_suffix("|")
                .ok_or_else(|| std::io::Error::other("missing '|' suffix delimiter"))?;

            buf.set_string(0, usize_to_u16(row)?, line, Style::default());
        }

        Ok(buf)
    }
}
