use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// manually set an API key for this command only
    #[arg(short, long)]
    pub key: Option<String>,

    /// save an API key to the config file
    #[arg(long)]
    pub storekey: Option<String>,

    /// choose a model to save to the config file
    #[arg(short, long)]
    pub model: Option<String>,

    /// Number of times to greet
    #[arg(short, long)]
    pub ignorelist: Option<String>,

    /// Focus a path to scan
    #[arg(short, long)]
    pub path: Option<String>,

    /// Outputs the worth-deleting files in a JSON file rather than showing them in the TUI selector
    #[arg(short, long)]
    pub json: Option<String>,

    /// Forces action that are not safe (like overwritting a  JSON file)
    #[arg(short, long)]
    pub force: bool,

    /// Doesn't ask Y/N questions (doesn't inlude the TUI selector)
    #[arg(short, long)]
    pub yes: bool,

    /// scan
    #[arg(long)]
    pub scan: bool,
}

pub fn parse_args() -> Args {
    Args::parse()
}

pub fn get_args_number() -> usize {
    std::env::args().count() - 1
}
