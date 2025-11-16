use crossterm::event::{Event::Key, EventStream, KeyEvent, KeyEventKind::Press};
use futures::StreamExt as _;
use ratatui_rseq::TerminalSession;

use crate::cli::Options;
use crate::handler::Handler as _;
use crate::log;
use crate::ui::UI;

/// Run the full interactive app, using the process arguments
pub async fn run() -> std::io::Result<()> {
    let opts = Options::parse();
    log::init(opts.log_path)?;
    run_terminal_session().await?;
    Ok(())
}

async fn run_terminal_session() -> std::io::Result<()> {
    let mut term = TerminalSession::start()?;
    let mut ui = UI::default();
    // let mut events = EventStream::new();

    term.draw(&ui)?;
    /*
    while let Some(event) = events.next().await.transpose()? {
        // ignore key event kind besides Press:
        if matches!(event, Key(KeyEvent { kind, .. }) if kind != Press) {
            continue;
        }

        if !ui.handle(event).await? {
            break;
        }
        term.draw(&ui)?;
    }
    */
    Ok(())
}
