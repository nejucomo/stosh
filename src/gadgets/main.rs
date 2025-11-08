use crossterm::event::Event;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Style, Stylize as _};
use ratatui::widgets::{Block, BorderType, Borders, Clear, WidgetRef};

use crate::{EventHandler, Gadget};

/// The main gadget for the whole UI
#[derive(Debug)]
pub struct MainPane {
    block: Block<'static>,
}

impl Default for MainPane {
    fn default() -> Self {
        Self {
            block: Block::new()
                .title("══╡ partish ╞")
                .title_style(Style::new().dark_gray())
                .borders(Borders::TOP)
                .border_type(BorderType::Double)
                .border_style(Style::new().dark_gray()),
        }
    }
}

impl Gadget for MainPane {}

impl WidgetRef for MainPane {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        Clear.render_ref(area, buf);
        self.block.render_ref(area, buf);

        //     let blockarea = block.inner(frame.area());
        //     frame.render_widget(block, frame.area());

        //     let items = ["Item 1", "Item 2", "Item 3"];
        //     let list = List::new(items)
        //         .block(
        //             Block::bordered()
        //                 .title("List")
        //                 .padding(Padding::symmetric(2, 1)),
        //         )
        //         .style(Style::new().white())
        //         .highlight_style(Style::new().italic())
        //         .highlight_symbol(">>")
        //         .repeat_highlight_symbol(true)
        //         .direction(ListDirection::BottomToTop);

        //     frame.render_widget(list, layouts::centered(blockarea, 8, 5));
        // })?;
    }
}

impl EventHandler for MainPane {
    type EventResult = ();

    fn handle_event(&mut self, event: Event) -> std::io::Result<()> {
        Err(std::io::Error::other(format!("{event:#?}")))
    }
}
