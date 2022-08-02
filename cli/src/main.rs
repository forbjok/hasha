use std::path::PathBuf;

use clap::Parser;

mod command;
mod ui;

use kecs::checksum_set::HashType;
use tracing::debug;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

use crate::ui::fancy::FancyUiHandler;

#[derive(Debug, Parser)]
#[clap(name = "KeCS", version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"))]
struct Opt {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Parser)]
enum Command {
    #[clap(about = "Generate a checksum set")]
    Generate {
        #[clap(help = "Path to generate checksum set for")]
        path: PathBuf,
        #[clap(long = "root-path", short = 'r', help = "Root path")]
        root_path: Option<PathBuf>,
        #[clap(long = "output", short = 'o', help = "Output file path")]
        output_file: Option<PathBuf>,
        #[clap(long = "hash-type", short = 't', help = "Specify hash type")]
        hash_type: Option<HashType>,
    },

    #[clap(about = "Compare differences between two checksum sets")]
    Diff {
        #[clap(help = "Checksum set to compare")]
        checksums_a_path: PathBuf,
        #[clap(help = "Checksum set to compare with")]
        checksums_b_path: PathBuf,
    },

    #[clap(about = "Verify checksums")]
    Verify {
        #[clap(help = "Path to checksum set file to verify")]
        checksums_path: PathBuf,
        #[clap(
            long = "root-path",
            short = 'r',
            help = "Specify root path (defaults to parent directory of checksum file)"
        )]
        root_path: Option<PathBuf>,
    },
}

fn main() -> Result<(), anyhow::Error> {
    let opt = Opt::parse();

    // Initialize logging
    initialize_logging();

    debug!("Debug logging enabled.");

    let mut ui = FancyUiHandler::new();

    match opt.command {
        Command::Generate {
            path,
            root_path,
            output_file,
            hash_type,
        } => command::generate(&path, output_file.as_deref(), root_path.as_deref(), hash_type, &mut ui)?,
        Command::Diff {
            checksums_a_path,
            checksums_b_path,
        } => command::diff(&checksums_a_path, &checksums_b_path, &mut ui)?,
        Command::Verify {
            checksums_path,
            root_path,
        } => command::verify(&checksums_path, root_path.as_deref(), &mut ui)?,
    };

    ui.clear()?;

    Ok(())
}

fn initialize_logging() {
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("warn")))
        .with_writer(std::io::stderr)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Setting default tracing subscriber failed!");
}
