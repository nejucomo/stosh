use crossterm::event::EventStream;
use futures::{StreamExt as _, stream};
use ratatui_rseq::TerminalSession;
use tokio_command_multiplexer::CommandMultiplexer;

use crate::cli::Options;
use crate::event::InputEvent;
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
    let mut events = EventStream::new();
    let mut cmux = CommandMultiplexer::default();

    term.draw(&ui)?;
    while let Some(evres) = stream::select(
        (&mut events).map(InputEvent::try_from),
        (&mut cmux).map(InputEvent::from).map(Ok),
    )
    .next()
    .await
    {
        let ev = evres?;
        if !ui.handle(ev).await? {
            break;
        }

        term.draw(&ui)?;
    }
    Ok(())
}
