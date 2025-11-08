use std::sync::mpsc;

use crossterm::event;

use crate::{Display, EventHandler as _, Notification, UI};

/// Run the full interactive app, using the process arguments
pub fn run() -> std::io::Result<()> {
    let mut disp = Display::start();

    let (sender, recv) = mpsc::sync_channel(1024);
    let ui = UI::from(sender);

    // Read and notify console input events:
    ui.spawn(|ui| {
        loop {
            let ev = event::read()?;
            ui.notify(ev);
        }
    });

    disp.draw()?;
    for notif in std::iter::from_fn(|| recv.recv().ok()) {
        use Notification::*;

        match notif {
            Exit => {
                return Ok(());
            }

            ThreadError(error) => {
                return Err(error);
            }

            CrosstermEvent(ev) => {
                disp.handle_event(ev)?;
            }
        }

        disp.draw()?;
    }

    Ok(())
}
