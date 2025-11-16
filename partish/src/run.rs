use std::fs::File;
use std::path::Path;

use crossterm::event::{Event::Key, EventStream, KeyEvent, KeyEventKind::Press};
use futures::StreamExt as _;
use ratatui_rseq::TerminalSession;
use tracing::Level;

use crate::cli::Options;
use crate::handler::Handler as _;
use crate::ui::UI;

/// Run the full interactive app, using the process arguments
pub async fn run() -> std::io::Result<()> {
    let opts = Options::parse();
    init_log(opts.log_path).await?;
    run_terminal_session().await?;
    Ok(())
}

async fn run_terminal_session() -> std::io::Result<()> {
    let mut term = TerminalSession::start()?;
    let mut ui = UI::default();
    let mut events = EventStream::new();

    term.draw(&ui)?;
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
    Ok(())
}

async fn init_log<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    let log = File::create(path)?;
    tracing_subscriber::fmt()
        .with_writer(log)
        .pretty()
        .with_max_level(Level::DEBUG)
        .init();
    tracing::debug!("initialized log");
    Ok(())
}
