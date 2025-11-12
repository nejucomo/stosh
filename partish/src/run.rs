use crossterm::event::EventStream;
use futures::StreamExt as _;
use ratatui_rseq::TerminalSession;

use crate::handler::Handler as _;
use crate::ui::UI;

/// Run the full interactive app, using the process arguments
pub async fn run() -> std::io::Result<()> {
    let mut term = TerminalSession::start();
    let mut ui = UI::default();
    let mut events = EventStream::new();

    term.draw(&ui)?;
    while let Some(event) = events.next().await.transpose()? {
        if !ui.handle(event).await? {
            break;
        }
        term.draw(&ui)?;
    }
    Ok(())
}
