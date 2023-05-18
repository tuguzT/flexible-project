//! Flexible Project user backend microservice binary.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![forbid(unsafe_code)]

use std::time::Duration;

use anyhow::{Context, Result};
use futures::stream::StreamExt;
use lapin::{
    options::{BasicAckOptions, BasicConsumeOptions, BasicPublishOptions, QueueDeclareOptions},
    publisher_confirm::Confirmation,
    types::FieldTable,
    BasicProperties, Connection, ConnectionProperties,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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

    let publish_channel = connection
        .create_channel()
        .await
        .with_context(|| "failed to create AMQP channel")?;
    let consume_channel = connection
        .create_channel()
        .await
        .with_context(|| "failed to create AMQP channel")?;

    let queue = publish_channel
        .queue_declare(
            "hello",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await
        .with_context(|| "failed to declare a queue")?;
    tracing::info!(?queue, "Declared queue");

    let mut consumer = consume_channel
        .basic_consume(
            "hello",
            "my_consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;
    tokio::spawn(async move {
        while let Some(delivery) = consumer.next().await {
            let delivery = delivery.expect("error in consumer");
            let message = std::str::from_utf8(&delivery.data).expect("string data");
            tracing::info!(%message);
            delivery.ack(BasicAckOptions::default()).await.expect("ack");
        }
    });

    loop {
        let confirm = publish_channel
            .basic_publish(
                "",
                "hello",
                BasicPublishOptions::default(),
                "Hello World!".as_bytes(),
                BasicProperties::default(),
            )
            .await?
            .await?;
        assert_eq!(confirm, Confirmation::NotRequested);
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
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
