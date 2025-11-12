use ratatui::text::Text;
use ratatui_rctx::TerminalSession;

fn main() -> std::io::Result<()> {
    let mut term = TerminalSession::start();
    term.draw(Text::raw("Hello World!"))?;
    std::thread::sleep(std::time::Duration::from_millis(2000));
    Ok(())
}
