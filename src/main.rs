use anyhow::Result;
use clap::Parser;
use log::debug;
use melange::{cli::args::Cli, config};

fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();
    let project_rules = config::project_rules::load_project_rules()?;

    debug!("Starting melange");

    Ok(())
}
