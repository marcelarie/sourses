use clap::Parser;

mod backend;
mod cli;

use cli::{Cli, Commands};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let mut backend = backend::create_backend();

    match cli.command {
        Commands::Record { name: _ } => {
            backend.spawn_shell()?;
        }
        Commands::Filter {
            types: _,
            regex: _,
            since: _,
        } => {
            // ... filtering logic ...
        }
    }

    Ok(())
}
