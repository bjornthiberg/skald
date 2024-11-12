mod cli;
mod journal;

use clap::Parser;
use cli::Cli;
use journal::Storage;
use std::io::Result;
use std::path::PathBuf;

fn main() -> Result<()> {
    let mut storage: Storage = Storage::new(PathBuf::from("journal_data"))?;

    let cli: Cli = Cli::parse();

    match cli.command {
        Some(command) => command.execute(&mut storage)?,
        None => println!("No command provided. Use --help for usage information."),
    }

    Ok(())
}
