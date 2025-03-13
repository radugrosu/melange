use anyhow::Result;
use clap::Parser;
use log::debug;
use melange::{
    cli::args::Cli,
    engine::llm_engine::LlmEngine,
    parser::{
        rust_parser::parse_rust_file,
        structure::{parse_crate, print_module_tree},
    },
};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    debug!("Starting up");

    let cli = Cli::parse();
    let llm = LlmEngine::from_config("melange-config.toml").unwrap();
    // let project_rules = config::project_rules::load_project_rules()?;
    for path in cli.files {
        debug!("Checking file: {}", path);
        // let (rules, modules) = parse_rust_file(&path);
        let crate_structure = parse_crate(".");
        print_module_tree(&crate_structure);
        // for rule in rules {
        //     let response = llm.query_with_rule(rule).await.unwrap();
        //     println!("{}", response);
        // }
    }
    Ok(())
}
