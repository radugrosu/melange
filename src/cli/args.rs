use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Files to check
    #[arg(short, long, value_name = "FILES")]
    pub files: Vec<String>,
    /// Directories to check
    #[arg(short, long)]
    pub dir: Option<String>,
    /// Level of verbosity
    #[arg(short, long, value_enum, default_value_t = Verbosity::Normal)]
    pub verbose: Verbosity,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Verbosity {
    Quiet,
    Normal,
    Verbose,
}
