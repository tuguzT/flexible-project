//! Flexible Project user backend microservice binary.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![forbid(unsafe_code)]

use anyhow::{Context, Result};
use lapin::{Connection, ConnectionProperties};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Entry point of the user backend microservice binary.
#[tokio::main]
pub async fn main() -> Result<()> {
    if cfg!(debug_assertions) {
        dotenv::dotenv().with_context(|| ".env file not found")?;
    }

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .try_init()
        .with_context(|| "failed to init tracing subscriber")?;

    let uri = std::env::var("AMQP_SERVER_URI").with_context(|| "AMQP_SERVER_URI must be set")?;
    let options = connection_properties();
    let _connection = Connection::connect(&uri, options)
        .await
        .with_context(|| "failed to connect to an AMQP server")?;
    tracing::info!("connected to an AMQP server");

    Ok(())
}

#[cfg(unix)]
fn connection_properties() -> ConnectionProperties {
    ConnectionProperties::default()
        .with_executor(tokio_executor_trait::Tokio::current())
        .with_reactor(tokio_reactor_trait::Tokio)
}

#[cfg(not(unix))]
fn connection_properties() -> ConnectionProperties {
    ConnectionProperties::default().with_executor(tokio_executor_trait::Tokio::current())
}
