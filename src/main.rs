use anyhow::Result;
use clap::Parser;
use log::debug;
use melange::{
    cli::args::Cli, engine::llm_engine::LlmEngine, parser::rust_parser::parse_rust_file,
};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    debug!("Starting up");

    let cli = Cli::parse();
    let llm = LlmEngine::from_config("./tests/fixtures/melange-config.toml").unwrap();
    // let project_rules = config::project_rules::load_project_rules()?;
    for path in cli.files {
        debug!("Checking file: {}", path);
        let rules = parse_rust_file(&path);
        debug!("Found {} rules", rules.len());
        for rule in rules {
            let response = llm.query_with_rule(rule).await.unwrap();
            println!("{}", response);
        }
    }
    Ok(())
}
