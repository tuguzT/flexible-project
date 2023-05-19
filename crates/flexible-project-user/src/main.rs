//! Flexible Project user backend microservice binary.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![forbid(unsafe_code)]

use std::{pin::pin, str};

use anyhow::{Context, Result};
use futures::{FutureExt, StreamExt};
use lapin::{
    options::{BasicConsumeOptions, BasicQosOptions, QueueDeclareOptions},
    types::FieldTable,
    Connection, ConnectionProperties,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::request::Request;

pub mod model;
pub mod request;

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
    let connection = Connection::connect(&uri, options)
        .await
        .with_context(|| "failed to connect to an AMQP server")?;
    tracing::info!("connected to an AMQP server");

    let channel = connection
        .create_channel()
        .await
        .with_context(|| "failed to create AMQP channel")?;

    let _queue = channel
        .queue_declare(
            "user",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await
        .with_context(|| "failed to declare user queue")?;

    channel
        .basic_qos(1, BasicQosOptions::default())
        .await
        .with_context(|| "failed to set channel prefetch count")?;

    let consumer = channel
        .basic_consume(
            "user",
            "user_service",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
        .with_context(|| "failed to create incoming requests consumer")?;
    tracing::info!("Listening for incoming requests...");

    let request_handler = consumer.for_each_concurrent(1, |delivery| async move {
        let delivery = match delivery {
            Ok(delivery) => delivery,
            Err(error) => {
                tracing::error!(%error, "message cannot be received");
                return;
            }
        };
        let data = delivery.data.as_slice();
        if let Ok(data) = str::from_utf8(data) {
            tracing::info!(%data, "received data from the message");
        }
        let request: Request = match serde_json::from_slice(data) {
            Ok(request) => request,
            Err(error) => {
                tracing::error!(%error, "message is not a valid request");
                return;
            }
        };
        tracing::info!(?request, "received request from the message");
        // TODO handle request and answer on it
    });
    let graceful_shutdown = shutdown_signal();

    let mut request_handler = pin!(request_handler.fuse());
    let mut graceful_shutdown = pin!(graceful_shutdown.fuse());
    futures::select! {
        () = request_handler => unreachable!("should listen for incoming requests endlessly"),
        () = graceful_shutdown => tracing::info!("gracefully shutdown the server"),
    }

    Ok(())
}

/// Declare Tokio-specific connection properties.
#[cfg(unix)]
fn connection_properties() -> ConnectionProperties {
    ConnectionProperties::default()
        .with_executor(tokio_executor_trait::Tokio::current())
        .with_reactor(tokio_reactor_trait::Tokio)
}

/// Declare Tokio-specific connection properties.
#[cfg(not(unix))]
fn connection_properties() -> ConnectionProperties {
    ConnectionProperties::default().with_executor(tokio_executor_trait::Tokio::current())
}

/// Catches a signal to shut down the server.
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("expect tokio signal ctrl-c")
}
