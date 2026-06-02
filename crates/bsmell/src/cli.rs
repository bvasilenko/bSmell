use crate::{BsmellError, SmellCategory, scan};
use clap::{Parser, Subcommand};
use std::{path::PathBuf, process::ExitCode};

#[derive(Debug, Parser)]
#[command(name = "bsmell")]
#[command(
    about = "CLI deflection-pattern detector. Reads session text; emits flagged-pattern directive."
)]
pub struct BsmellCli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Scan(ScanArgs),
    Categories,
    Update,
    Init,
    Tail,
    Explain,
}

#[derive(Debug, Clone, Eq, PartialEq, clap::Args)]
pub struct ScanArgs {
    #[arg(long, value_name = "path-or-fd")]
    pub session: Option<PathBuf>,
    #[arg(long, value_name = "path")]
    pub diff: Option<PathBuf>,
    #[arg(long, value_name = "path")]
    pub manifest: Option<PathBuf>,
    #[arg(long)]
    pub json: bool,
    #[arg(long)]
    pub quiet: bool,
    #[arg(long, value_name = "text")]
    pub reason: Option<String>,
}

impl BsmellCli {
    pub fn run(self) -> Result<ExitCode, BsmellError> {
        match self.command {
            Command::Scan(args) => scan::run(args),
            Command::Categories => {
                for category in SmellCategory::ALL {
                    println!("{category}");
                }
                Ok(ExitCode::SUCCESS)
            }
            Command::Update => placeholder("update"),
            Command::Init => placeholder("init"),
            Command::Tail => placeholder("tail"),
            Command::Explain => placeholder("explain"),
        }
    }
}

fn placeholder(command_name: &str) -> Result<ExitCode, BsmellError> {
    // Deferred to a later bsmell package cycle so public commands stay callable while behavior is pending.
    println!("bsmell {command_name} placeholder: behavior is deferred.");
    Ok(ExitCode::SUCCESS)
}
