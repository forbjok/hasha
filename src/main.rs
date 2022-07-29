use std::path::PathBuf;

use clap::Parser;

mod checksum_set;
mod command;
mod error;
mod util;

use tracing::debug;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

#[derive(Debug, Parser)]
#[clap(name = "Hasha", version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"))]
struct Opt {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Parser)]
enum Command {
    #[clap(about = "Calculate a checksum set")]
    Calculate {
        #[clap(help = "Path to calculate checksum set for")]
        path: PathBuf,
        #[clap(long = "root-path", short = 'r', help = "Output file path")]
        root_path: Option<PathBuf>,
        #[clap(long = "output", short = 'o', help = "Output file path")]
        output_file: Option<PathBuf>,
    },

    #[clap(about = "Compare differences between two checksum sets")]
    Diff {
        #[clap(help = "Checksum set to compare")]
        a: PathBuf,
        #[clap(help = "Checksum set to compare with")]
        b: PathBuf,
    },
}

fn main() {
    let opt = Opt::parse();

    // Initialize logging
    initialize_logging();

    debug!("Debug logging enabled.");

    let cmd_result = match opt.command {
        Command::Calculate {
            path,
            root_path,
            output_file,
        } => command::calculate(path, output_file, root_path),
        Command::Diff { a, b } => command::diff(a, b),
    };

    match cmd_result {
        Ok(_) => {}
        Err(err) => {
            // Print error description to stderr
            eprintln!("{}", err.description);

            // Return the exit code that corresponds to the error kind
            std::process::exit(err.kind.exit_code());
        }
    };
}

fn initialize_logging() {
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("warn")),
        )
        .with_writer(std::io::stderr)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Setting default tracing subscriber failed!");
}
