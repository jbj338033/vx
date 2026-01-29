mod cli;
mod commands;
mod error;
mod ffmpeg;
mod utils;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Gif(args) => commands::gif::execute(args),
        Commands::Compress(args) => commands::compress::execute(args),
        Commands::To(args) => commands::convert::execute(args),
        Commands::Info(args) => commands::info::execute(args),
    }
}
