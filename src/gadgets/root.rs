use crossterm::event::{Event, KeyCode};
use ratatui::widgets::Clear;

use crate::gadgets::{ExitDialog, MainPane};
use crate::{ContextualWidget, EventHandler, Gadget, RenderContext, UI};

/// The root [Gadget] encompassing all rendering and [Event] handling
///
/// In addition to simply wrapping the [MainPane], this gadget supports the `ExitDialog`
#[derive(Debug)]
pub struct RootGadget {
    ui: UI,
    mp: MainPane,
    dialog: Option<ExitDialog>,
}

impl RootGadget {
    /// Construct with the [UI] notifier
    pub fn new(ui: UI) -> Self {
        Self {
            ui: ui.clone(),
            mp: MainPane::new(ui),
            dialog: None,
        }
    }
}

impl Gadget for RootGadget {}

impl ContextualWidget for &RootGadget {
    fn render_to_context<'b>(self, ctx: &mut RenderContext<'b>) {
        ctx.render(Clear).render(&self.mp).render(&self.dialog);
    }
}

impl EventHandler for RootGadget {
    type EventResult = ();

    fn handle_event(&mut self, event: Event) -> std::io::Result<()> {
        use Event::Key;

        if let Some(dialog) = self.dialog.as_mut() {
            if !dialog.handle_event(event)? {
                self.dialog = None;
            }
            Ok(())
        } else if matches!(event, Key(kev) if kev.code == KeyCode::Esc) {
            self.dialog = Some(ExitDialog::new(self.ui.clone()));
            Ok(())
        } else {
            self.mp.handle_event(event)
        }
    }
}
