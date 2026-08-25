mod app_state;
mod protocol;
mod routes;
mod server;
mod session;
mod session_manager;
mod ws_error;
mod ws_handler;

use std::net::SocketAddr;
use std::sync::Arc;

use dice_rs::service::manager::DiceManager;
use tracing::Level;
use tracing_subscriber::fmt;

use crate::app_state::AppState;
use crate::server::Server;

/// Parse command-line arguments for bind address and verbosity.
struct Args {
    bind_address: SocketAddr,
    verbose: u8,
}

impl Args {
    fn parse() -> Self {
        let mut args = std::env::args().skip(1);
        let mut bind_address: SocketAddr = "0.0.0.0:3000".parse().expect("default bind address");
        let mut verbose: u8 = 0;

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--bind" | "-b" => {
                    if let Some(addr) = args.next() {
                        bind_address = addr.parse().expect("valid socket address");
                    }
                }
                "--verbose" | "-v" => {
                    verbose = verbose.saturating_add(1);
                }
                _ => {}
            }
        }

        Self {
            bind_address,
            verbose,
        }
    }
}

fn init_logging(verbose: u8) {
    let level = match verbose {
        0 => Level::WARN,
        1 => Level::INFO,
        2 => Level::DEBUG,
        _ => Level::TRACE,
    };
    fmt().with_max_level(level).with_target(false).init();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    init_logging(args.verbose);

    let manager = Arc::new(DiceManager::new().await?);
    let state = Arc::new(AppState::new(manager));
    let server = Server::new(state, args.bind_address);

    server.run().await?;

    Ok(())
}
