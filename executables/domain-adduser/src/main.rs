use std::process;
use clap::{CommandFactory, Parser};

/// 
/// Command line arguments
/// 
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input file name
    #[arg(short, long)]
    filename: Option<String>,

    /// Interactive option
    #[arg(short, long)]
    interactive: Option<bool>,

    /// Update
    #[arg(short, long)]
    update: Option<bool>
}

fn main() {
    // Parse command line arguments
    let cli = Args::parse();

    // If no command has been provided, exit
    if cli.filename.is_none() && cli.interactive.unwrap_or(false) == false && cli.update.unwrap_or(false) == false {
        eprintln!("{}", Args::command().render_usage());
        process::exit(1)
    }


}
