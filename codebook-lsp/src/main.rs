mod lsp;
use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};
use log::info;
use lsp::Backend;
use tower_lsp::{LspService, Server};

#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FOLDER")]
    cache_dir: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Serve the Language Server
    Serve {},
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Initialize logging so we can see server messages in the console.
    env_logger::init();
    let cli = Cli::parse();

    let cache_dir = match cli.cache_dir.as_deref() {
        Some(path) => path,
        None => Path::new(".cache/dictionaries/"),
    };
    match &cli.command {
        Some(Commands::Serve {}) => {
            serve_lsp(cache_dir).await;
        }
        None => {}
    }
}

async fn serve_lsp(cache_dir: &Path) {
    info!("Starting SpellCheck Language Server...");
    let (stdin, stdout) = (tokio::io::stdin(), tokio::io::stdout());
    let (service, socket) = LspService::new(|client| Backend::new(client, cache_dir.to_path_buf()));
    Server::new(stdin, stdout, socket).serve(service).await;
}
