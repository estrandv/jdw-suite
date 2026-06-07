#![feature(result_flattening, proc_macro_hygiene, decl_macro)]

use clap::{CommandFactory, Parser, Subcommand};

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
    /// Play a composition file on the running suite (queue update)
    Play {
        /// Path to a billboard composition file (.txt or .bbd)
        file: String,
    },
    /// Configure synths and samples for a composition (setup + commands)
    Setup {
        /// Path to a billboard composition file
        file: String,
    },
    /// Re-configure a composition (commands only, no synth reload)
    Update {
        /// Path to a billboard composition file
        file: String,
    },
    /// Stop all playback on the running suite
    Stop,
    /// Stop playback and silence all drones
    Quiet {
        /// Path to a billboard composition file (for drone identification)
        file: String,
    },
    /// Shut down the running suite
    Terminate,
}

fn main() {
    let cli = Cli::parse();
    let quiet = cli.quiet;

    let Some(command) = cli.command else {
        let mut cmd = Cli::command();
        cmd.print_help().unwrap();
        println!();
        std::process::exit(0);
    };

    match command {
        Commands::All => launch::run_all(quiet),
        Commands::Router => launch::run_router(quiet),
        Commands::Sc => launch::run_sc(quiet),
        Commands::Sequencer => launch::run_sequencer(quiet),
        Commands::Play { file } => client::play(&file),
        Commands::Setup { file } => client::setup(&file),
        Commands::Update { file } => client::update(&file),
        Commands::Stop => client::stop(),
        Commands::Quiet { file } => client::quiet(&file),
        Commands::Terminate => client::terminate(),
    }
}
