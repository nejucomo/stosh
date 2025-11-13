use crossterm::event::{Event::Key, EventStream, KeyEvent, KeyEventKind::Press};
use futures::StreamExt as _;
use ratatui_rseq::TerminalSession;

use crate::handler::Handler as _;
use crate::ui::UI;

/// Run the full interactive app, using the process arguments
pub async fn run() -> std::io::Result<()> {
    let log_events = false;

    let mut evlog = if log_events { Some(vec![]) } else { None };

    let res = run_inner(&mut evlog).await;

    if let Some(evlog) = evlog {
        println!("Event Log: {evlog:#?}");
    }
    res
}

async fn run_inner(evlog: &mut Option<Vec<crossterm::event::Event>>) -> std::io::Result<()> {
    let mut term = TerminalSession::start()?;
    let mut ui = UI::default();
    let mut events = EventStream::new();

    term.draw(&ui)?;
    while let Some(event) = events.next().await.transpose()? {
        if let Some(v) = evlog.as_mut() {
            v.push(event.clone())
        }

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
