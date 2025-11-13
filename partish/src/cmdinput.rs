use crossterm::event::Event::{self, Key};
use crossterm::event::KeyCode::Enter;
use crossterm::event::{KeyEvent, KeyModifiers};
use ratatui::layout::Constraint::{Fill, Length};
use ratatui::style::{Style, Stylize};
use ratatui::text::Line;
use ratatui::widgets::Widget;
use ratatui_rseq::Renderable;
use tui_textarea::TextArea;

use crate::handler::Handler;
use crate::u16util::IntoU16 as _;

#[derive(Debug)]
pub(crate) struct CommandInput {
    ta: TextArea<'static>,
}

impl CommandInput {
    /// The height of the CommandInput
    pub(crate) fn height(&self) -> usize {
        self.ta.lines().len()
    }

    fn new_text_area() -> TextArea<'static> {
        let mut ta = TextArea::default();
        ta.set_cursor_style(Style::reset().on_light_cyan());
        ta.set_cursor_line_style(Style::default());
        ta.set_style(Style::reset().gray().on_dark_gray());
        ta
    }

    fn pop_text(&mut self) -> String {
        std::mem::replace(&mut self.ta, Self::new_text_area())
            .into_lines()
            .into_iter()
            .map(|mut s| {
                s.push('\n');
                s
            })
            .collect()
    }
}

impl Default for CommandInput {
    fn default() -> Self {
        Self {
            ta: Self::new_text_area(),
        }
    }
}

impl Renderable for &CommandInput {
    fn into_widget(self) -> impl Widget {
        let prompt = Line::from("⟨0⟩".black().on_light_cyan());
        let pwidth = prompt.width().into_u16();

        prompt
            .constrained(Length(pwidth))
            .on_left()
            .followed_by(self.ta.constrained(Fill(1)))
            .horizontal_margin(1)
            .spacing(1)
    }
}

impl Handler<Event> for CommandInput {
    type Response = std::io::Result<Option<String>>;

    async fn handle(&mut self, ev: Event) -> Self::Response {
        match ev {
            Key(KeyEvent {
                code: Enter,
                modifiers,
                ..
            }) => {
                if modifiers.is_empty() {
                    Ok(Some(self.pop_text()))
                } else if modifiers == KeyModifiers::CONTROL {
                    self.ta.insert_newline();
                    Ok(None)
                } else {
                    Ok(None)
                }
            }
            ev => {
                self.ta.input(ev);
                Ok(None)
            }
        }
    }
}
