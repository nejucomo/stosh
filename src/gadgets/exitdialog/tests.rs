use indoc::indoc;
use ratatui::buffer::{Buffer, Cell};
use ratatui::layout::Rect;
use ratatui::style::{Style, Stylize as _};
use ratatui::widgets::WidgetRef as _;

use crate::UI;
use crate::gadgets::ExitDialog;
use crate::testutils::BufferExt as _;

#[test]
fn test_render() {
    let expected = {
        let mut buf = Buffer::parse(indoc! { r#"
            |                    |
            |                    |
            |                    |
            |   ┌─────────────┐  |
            |   │             │  |
            |   │  Exit? y/n  │  |
            |   │             │  |
            |   └─────────────┘  |
            |                    |
            |                    |
        "# })
        .unwrap();

        buf.set_style(
            Rect::new(6, 5, 9, 1),
            Style::new().bold().white().on_black(),
        );
        buf
    };

    let (ui, _) = UI::create_channel();
    let ed = ExitDialog::new(ui);
    let mut actual = Buffer {
        area: expected.area,
        content: vec![Cell::default(); usize::try_from(expected.area.area()).unwrap()],
    };
    ed.render_ref(expected.area, &mut actual);
    assert_eq!(expected, actual);
}
