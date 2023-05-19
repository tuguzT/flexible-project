//! Utilities for user service setup.

use anyhow::{Context, Result};
use lapin::{
    options::{BasicConsumeOptions, BasicQosOptions, QueueDeclareOptions},
    types::FieldTable,
    Channel, Connection, ConnectionProperties, Consumer, Queue,
};

/// Declare Tokio-specific connection properties.
#[cfg(not(unix))]
pub fn connection_properties() -> ConnectionProperties {
    ConnectionProperties::default().with_executor(tokio_executor_trait::Tokio::current())
}

/// Declare Tokio-specific connection properties.
#[cfg(unix)]
pub fn connection_properties() -> ConnectionProperties {
    ConnectionProperties::default()
        .with_executor(tokio_executor_trait::Tokio::current())
        .with_reactor(tokio_reactor_trait::Tokio)
}

/// Creates connection to AMQP server.
pub async fn create_connection(uri: &str) -> Result<Connection> {
    let options = connection_properties();
    let connection = Connection::connect(uri, options)
        .await
        .with_context(|| "failed to connect to an AMQP server")?;
    Ok(connection)
}

/// Creates channel from AMQP server connection.
pub async fn create_channel(connection: &Connection) -> Result<Channel> {
    let channel = connection
        .create_channel()
        .await
        .with_context(|| "failed to create AMQP channel")?;
    channel
        .basic_qos(1, BasicQosOptions::default())
        .await
        .with_context(|| "failed to set channel prefetch count")?;
    Ok(channel)
}

/// Declares queue from AMQP server connection.
pub async fn declare_queue(channel: &Channel) -> Result<Queue> {
    let options = QueueDeclareOptions {
        durable: true,
        ..Default::default()
    };
    let queue = channel
        .queue_declare("user", options, FieldTable::default())
        .await
        .with_context(|| "failed to declare user queue")?;
    Ok(queue)
}

/// Creates message consumer from AMQP server connection.
pub async fn create_consumer(channel: &Channel) -> Result<Consumer> {
    let consumer = channel
        .basic_consume(
            "user",
            "user_service",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
        .with_context(|| "failed to create incoming requests consumer")?;
    Ok(consumer)
}
