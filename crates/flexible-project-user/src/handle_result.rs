//! Utilities for handling result of request handling.

use lapin::{
    message::Delivery,
    options::{BasicAckOptions, BasicNackOptions, BasicRejectOptions},
    Channel,
};

use crate::handle_request::HandleRequestError;

/// Handle result of the request handling.
pub async fn handle_result(
    result: Result<(), HandleRequestError>,
    delivery: &Delivery,
    channel: &Channel,
) {
    let delivery_tag = delivery.delivery_tag;
    match result {
        Ok(_) => {
            let options = BasicAckOptions::default();
            let result = channel.basic_ack(delivery_tag, options).await;
            if let Err(error) = result {
                tracing::error!(%error, "failed to ack incoming request");
            }
        }
        Err(error) => match error {
            HandleRequestError::Reject => {
                tracing::info!(%delivery_tag, "rejecting invalid request");
                let options = BasicRejectOptions::default();
                let result = channel.basic_reject(delivery_tag, options).await;
                if let Err(error) = result {
                    tracing::error!(%error, "failed to reject invalid request");
                }
            }
            HandleRequestError::Nack => {
                tracing::info!(%delivery_tag, "not acknowledge request");
                let options = BasicNackOptions {
                    requeue: true,
                    ..Default::default()
                };
                let result = channel.basic_nack(delivery_tag, options).await;
                if let Err(error) = result {
                    tracing::error!(%error, "failed to not acknowledge request");
                }
            }
        },
    }
}
