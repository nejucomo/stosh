use ratatui::layout::Rect;

pub(crate) trait RectExt: Sized {
    fn split_vertically(self, height: u16) -> (Self, Self);
}

impl RectExt for Rect {
    fn split_vertically(self, height: u16) -> (Self, Self) {
        let Rect {
            x,
            y,
            width,
            height: total_height,
        } = self;

        let h = std::cmp::min(total_height, height);

        (
            Rect::new(x, y, width, h),
            Rect::new(x, y + h, width, total_height - h),
        )
    }
}
