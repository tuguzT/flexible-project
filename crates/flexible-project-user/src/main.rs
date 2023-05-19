//! Flexible Project user backend microservice binary.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![forbid(unsafe_code)]

use std::pin::pin;

use anyhow::{Context, Result};
use futures::{FutureExt, StreamExt};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use self::{
    handle_request::handle_request,
    handle_result::handle_result,
    setup::{create_channel, create_connection, create_consumer, declare_queue},
};

pub mod handle_request;
pub mod handle_result;
pub mod model;
pub mod request;
pub mod setup;

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
    let connection = create_connection(&uri).await?;
    tracing::info!("connected to an AMQP server");

    let channel = create_channel(&connection).await?;
    let _queue = declare_queue(&channel).await?;
    let consumer = create_consumer(&channel).await?;
    tracing::info!("Listening for incoming requests...");

    let request_handler = consumer.for_each_concurrent(1, |delivery| async {
        let delivery = match delivery {
            Ok(delivery) => delivery,
            Err(error) => {
                tracing::error!(%error, "message cannot be received");
                return;
            }
        };
        let result = handle_request(&delivery, &channel).await;
        handle_result(result, &delivery, &channel).await
    });
    let graceful_shutdown = shutdown_signal();

    let mut request_handler = pin!(request_handler.fuse());
    let mut graceful_shutdown = pin!(graceful_shutdown.fuse());
    futures::select! {
        () = request_handler => tracing::error!("should listen for incoming requests endlessly"),
        () = graceful_shutdown => tracing::info!("gracefully shutdown the server"),
    }
    Ok(())
}

/// Catches a signal to shut down the server.
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("expect tokio signal ctrl-c")
}
