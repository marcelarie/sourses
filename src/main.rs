// src/main.rs

use std::sync::mpsc::channel;

use anyhow::Result;
use clap::Parser;

mod backend;
mod cli;
mod db;

use backend::create_backend;
use cli::{Cli, Commands};
use db::{get_connection, initialize_schema, insert_item};

fn main() -> Result<()> {
    let cli = Cli::parse();

    let conn = get_connection()?;
    initialize_schema(&conn)?;

    match cli.command {
        Commands::Record { name: _ } => {
            // 1. Create an mpsc channel for PTY I/O
            let (tx, rx) = channel::<Vec<u8>>();

            // 2. Spawn the shell under PTY, giving it the sender
            let mut backend = create_backend();
            backend.spawn_shell(tx)?;

            // 3. In main thread, read bytes, accumulate into lines
            let mut buffer = String::new();
            for chunk in rx {
                // append new bytes
                buffer.push_str(&String::from_utf8_lossy(&chunk));

                // process full lines
                while let Some(idx) = buffer.find('\n') {
                    let line: String = buffer.drain(..=idx).collect();
                    let line = line.trim_end(); // drop the '\n'

                    // 4. Run your regex extractors here. For simplicity, store the whole line:
                    insert_item(&conn, "output", line)?;
                }
            }
        }
        Commands::Filter {
            types: _,
            regex: _,
            since: _,
        } => {
            // 1. Prepare a trivial query: pull every item in timestamp descending order.
            let mut stmt = conn.prepare(
                "SELECT type, text
                 FROM items
                 ORDER BY timestamp DESC",
            )?;

            // 2. Execute and iterate over each row.
            let rows = stmt.query_map([], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })?;

            // 3. Print each result to stdout.
            for result in rows {
                let (ty, text) = result?;
                println!("[{}] {}", ty, text);
            }
        }
    }

    Ok(())
}
