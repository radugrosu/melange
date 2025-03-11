use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, value_name = "FILES")]
    pub files: Vec<String>,
    #[arg(short, long)]
    pub dir: Option<String>,
}
