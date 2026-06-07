#![feature(result_flattening, proc_macro_hygiene, decl_macro)]

use clap::{Parser, Subcommand};

mod client;
mod config;
mod launch;

#[derive(Parser)]
#[command(name = "jdw", about = "JackDAW — modular music production suite")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short, long, global = true)]
    quiet: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Launch all services (router, sequencer, sc)
    All,
    /// Launch only the OSC message router
    Router,
    /// Launch only the SuperCollider wrapper
    Sc,
    /// Launch only the beat-synchronous sequencer
    Sequencer,
    /// Send a composition file to the running suite
    Send {
        /// Path to a billboard composition file (.txt or .bbd)
        file: String,
    },
    /// Stop all playback on the running suite
    Stop,
    /// Load synthdefs and samples for a composition
    Setup {
        /// Path to a billboard composition file
        file: String,
    },
    /// Shut down the running suite
    Terminate,
}

fn main() {
    let cli = Cli::parse();
    let quiet = cli.quiet;

    match cli.command.unwrap_or(Commands::All) {
        Commands::All => launch::run_all(quiet),
        Commands::Router => launch::run_router(quiet),
        Commands::Sc => launch::run_sc(quiet),
        Commands::Sequencer => launch::run_sequencer(quiet),
        Commands::Send { file } => client::send(&file),
        Commands::Stop => client::stop(),
        Commands::Setup { file } => client::setup(&file),
        Commands::Terminate => client::terminate(),
    }
}
