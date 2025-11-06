use std::sync::mpsc;

use crossterm::event;

use crate::{Display, Notification, UI};

/// Run the full interactive app, using the process arguments
pub fn run() -> std::io::Result<()> {
    use Notification::*;

    let mut disp = Display::new();

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
        match notif {
            ThreadError(error) => {
                return Err(error);
            }

            other => todo!("unhandled: {other:?}"),
        }

        // disp.draw()?;
    }

    Ok(())
}
