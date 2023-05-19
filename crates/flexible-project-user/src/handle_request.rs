//! Utilities to properly handle incoming request.

use std::str;

use lapin::{
    message::Delivery, options::BasicPublishOptions, types::ShortString, BasicProperties, Channel,
};

use crate::request::Request;

/// Type of error which is returned if request handling fails.
pub enum HandleRequestError {
    /// Reject incoming request (without requeueing).
    Reject,
    /// Not acknowledge incoming request (with requeueing).
    Nack,
}

/// Handles incoming request.
pub async fn handle_request(
    delivery: &Delivery,
    channel: &Channel,
) -> Result<(), HandleRequestError> {
    let routing_key = get_routing_key(delivery)?;
    let correlation_id = get_correlation_id(delivery)?;
    let payload = create_response(delivery).await?;

    let publish_response = publish_response(
        channel,
        payload.as_slice(),
        routing_key.as_str(),
        correlation_id.clone(),
    );
    publish_response.await
}

fn get_routing_key(delivery: &Delivery) -> Result<&ShortString, HandleRequestError> {
    let routing_key = delivery.properties.reply_to().as_ref().ok_or_else(|| {
        tracing::error!("missing `reply_to` property");
        HandleRequestError::Reject
    })?;
    Ok(routing_key)
}

fn get_correlation_id(delivery: &Delivery) -> Result<&ShortString, HandleRequestError> {
    let correlation_id = delivery
        .properties
        .correlation_id()
        .as_ref()
        .ok_or_else(|| {
            tracing::error!("missing `correlation_id` property");
            HandleRequestError::Reject
        })?;
    Ok(correlation_id)
}

fn get_request(delivery: &Delivery) -> Result<Request, HandleRequestError> {
    let data = delivery.data.as_slice();
    if let Ok(data) = str::from_utf8(data) {
        tracing::info!(%data, "received data from the message");
    }
    let request = serde_json::from_slice(data).map_err(|error| {
        tracing::error!(%error, "message is not a valid request");
        HandleRequestError::Reject
    })?;
    Ok(request)
}

async fn create_response(delivery: &Delivery) -> Result<Vec<u8>, HandleRequestError> {
    let request = get_request(delivery)?;
    tracing::info!(?request, "received request from the message");

    // TODO handle request and answer on it with actual data
    let payload = b"".to_vec();
    Ok(payload)
}

async fn publish_response(
    channel: &Channel,
    payload: &[u8],
    routing_key: &str,
    correlation_id: ShortString,
) -> Result<(), HandleRequestError> {
    let publish = channel.basic_publish(
        "",
        routing_key,
        BasicPublishOptions::default(),
        payload,
        BasicProperties::default().with_correlation_id(correlation_id.clone()),
    );
    if let Err(error) = publish.await {
        tracing::error!(%error, "failed to publish response into reply queue");
        return Err(HandleRequestError::Nack);
    }
    Ok(())
}
