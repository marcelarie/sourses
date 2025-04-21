use core::fmt;

use clap::{Parser, Subcommand, ValueEnum};

/// sourses: a PTY‑backed shell session indexer
#[derive(Debug, Parser)]
#[command(
    name = "sourses",
    version = "0.1.0",
    about = "Index and query your shell session via PTY"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Spawn a shell under a PTY and record its I/O
    Record {
        /// Optional session name override
        #[arg(short, long)]
        name: Option<String>,
    },

    /// Query the indexed session items
    Filter {
        /// Only show items of these types
        #[arg(short = 't', long = "type", value_enum)]
        types: Vec<ItemType>,

        /// Regex to further filter by text
        #[arg(short = 'r', long)]
        regex: Option<String>,

        /// Limit to items from the last N seconds/minutes/hours (e.g. "10m", "2h")
        #[arg(short, long)]
        since: Option<String>,
    },
}

#[derive(Debug, Clone, ValueEnum)]
pub enum ItemType {
    Url,
    Path,
    Command,
    Output,
    Env,
    Pid,
    Error,
    Clipboard,
    Tmux,
}

impl fmt::Display for ItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ItemType::Url => "url",
            ItemType::Path => "path",
            ItemType::Command => "command",
            ItemType::Output => "output",
            ItemType::Env => "env",
            ItemType::Pid => "pid",
            ItemType::Error => "error",
            ItemType::Clipboard => "clipboard",
            ItemType::Tmux => "tmux",
        };
        write!(f, "{}", s)
    }
}
