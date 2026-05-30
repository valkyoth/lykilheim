//! Lykilheim foundation server binary.

use std::{net::SocketAddr, path::PathBuf};

use clap::Parser;
use lykilheim::{api, config::Config, error::Result};
use tokio::{net::TcpListener, signal};
use tracing::info;

#[derive(Debug, Parser)]
#[command(name = "lykilheim")]
#[command(about = "Lykilheim secrets manager foundation server")]
struct Cli {
    /// Path to a TOML configuration file.
    #[arg(long)]
    config: Option<PathBuf>,

    /// Validate configuration and exit.
    #[arg(long)]
    check_config: bool,

    /// Override the configured listen address.
    #[arg(long)]
    listen: Option<SocketAddr>,
}

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing();

    let cli = Cli::parse();
    let mut config = Config::load_optional(cli.config.as_deref())?;
    if let Some(listen) = cli.listen {
        config.server.listen = listen;
    }
    config.validate()?;

    if cli.check_config {
        return Ok(());
    }

    let listen = config.server.listen;
    let app = api::router(api::AppState::from_config(config));
    let listener = TcpListener::bind(listen).await?;
    info!(%listen, "starting lykilheim");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

fn init_tracing() {
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "lykilheim=info,tower_http=info".into());

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_target(false)
        .compact()
        .init();
}

async fn shutdown_signal() {
    #[cfg(unix)]
    {
        tokio::select! {
            _ = signal::ctrl_c() => {}
            _ = terminate_signal() => {}
        }
    }

    #[cfg(not(unix))]
    {
        let _ = signal::ctrl_c().await;
    }
}

#[cfg(unix)]
async fn terminate_signal() {
    use tokio::signal::unix::{SignalKind, signal};

    let Ok(mut stream) = signal(SignalKind::terminate()) else {
        std::future::pending::<()>().await;
        return;
    };
    stream.recv().await;
}
