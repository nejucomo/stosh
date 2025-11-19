use crate::cli::Options;
use crate::{evloop, log};

/// Run the full interactive app, using the process arguments
pub async fn run() -> std::io::Result<()> {
    let opts = Options::parse();
    log::init(opts.log_path)?;
    evloop::run().await?;
    Ok(())
}
