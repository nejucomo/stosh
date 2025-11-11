use crossterm::event::{Event, KeyCode};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::WidgetRef;

use crate::gadgets::{ExitDialog, MainPane};
use crate::{EventHandler, Gadget, UI};

/// The root [Gadget] encompassing all rendering and [Event] handling
///
/// In addition to simply wrapping the [MainPane], this gadget supports the `ExitDialog`
#[derive(Debug)]
pub struct RootGadget {
    mp: MainPane,
    dialog: ExitDialog,
    dialog_active: bool,
}

impl RootGadget {
    /// Construct with the [UI] notifier
    pub fn new(ui: UI) -> Self {
        Self {
            mp: MainPane::new(ui.clone()),
            dialog: ExitDialog::new(ui),
            dialog_active: false,
        }
    }
}

impl Gadget for RootGadget {}

impl WidgetRef for RootGadget {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        self.mp.render_ref(area, buf);
        if self.dialog_active {
            self.dialog.render_ref(area, buf);
        }
    }
}

impl EventHandler for RootGadget {
    type EventResult = ();

    fn handle_event(&mut self, event: Event) -> std::io::Result<()> {
        use Event::Key;

        if self.dialog_active {
            self.dialog_active = self.dialog.handle_event(event)?;
            Ok(())
        } else if matches!(event, Key(kev) if kev.code == KeyCode::Esc) {
            self.dialog_active = true;
            Ok(())
        } else {
            self.mp.handle_event(event)
        }
    }
}
