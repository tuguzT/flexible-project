//! Notification data model of the gateway service.

use async_graphql::{InputObject, Object, SimpleObject, Subscription, ID};
use chrono::{DateTime, Utc};
use futures::{stream::Repeat, Stream};

/// Query object of notifications of the Flexible Project system.
#[derive(Debug, Default)]
pub struct NotificationQuery;

#[Object]
impl NotificationQuery {
    /// Retrieve a list of all notifications of the user received earlier.
    pub async fn received_notifications(&self, user_id: ID) -> Vec<Notification> {
        let _ = user_id;
        None.unwrap()
    }
}

/// Mutation object of notifications of the Flexible Project system.
#[derive(Debug, Default)]
pub struct NotificationMutation;

#[Object]
impl NotificationMutation {
    /// Mark notification from the input notification stream as received by the user.
    pub async fn receive_notification(&self, user_id: ID, notification_id: ID) -> Notification {
        let _ = (user_id, notification_id);
        None.unwrap()
    }

    /// Update properties of the notification properties of user by provided identifier with provided data.
    pub async fn update_notification_preferences(
        &self,
        user_id: ID,
        update: UpdateNotificationPreferences,
    ) -> NotificationPreferences {
        let _ = (user_id, update);
        None.unwrap()
    }
}

/// Subscription object of notifications of the Flexible Project system.
#[derive(Debug, Default)]
pub struct NotificationSubscription;

#[Subscription]
impl NotificationSubscription {
    /// Subscribe for all incoming notifications of the user by provided identifier.
    pub async fn incoming_notifications(&self, user_id: ID) -> impl Stream<Item = Notification> {
        let _ = user_id;
        None::<Repeat<_>>.unwrap()
    }
}

/// Notification properties of the Flexible project system.
#[derive(Debug, SimpleObject, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Notification {
    /// Unique identifier of the notification.
    pub id: ID,
    /// Type of the notification.
    pub type_id: ID,
    /// Message of the notification.
    pub message: String,
    /// Source of the notification (could be another user, workspace or project).
    pub source: ID,
    /// Destination of the notification (always user).
    pub destination: ID,
    /// Send time of the notification.
    pub send_time: DateTime<Utc>,
    /// Receive time of the notification.
    pub receive_time: DateTime<Utc>,
}

/// Notification type properties of the Flexible project system.
#[derive(Debug, SimpleObject, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NotificationType {
    /// Unique identifier of the notification type.
    pub id: ID,
    /// Unique name of the notification type.
    pub name: String,
    /// Description of the notification type.
    pub description: String,
}

/// Notification preferences properties of the Flexible Project system.
#[derive(Debug, SimpleObject, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NotificationPreferences {
    /// Identifier of the user which are these preferences of.
    pub user_id: ID,
    /// Set of preferences for each notification type.
    pub type_preferences: Vec<NotificationTypePreferences>,
}

/// Data of notification preferences to update.
#[derive(Debug, InputObject)]
pub struct UpdateNotificationPreferences {
    /// Set of preferences for each notification type to update, if present.
    pub type_preferences: Option<Vec<NotificationTypePreferencesInput>>,
}

/// Notification type preferences properties of the Flexible Project system.
#[derive(Debug, SimpleObject, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NotificationTypePreferences {
    /// Identifier of notification type.
    pub type_id: ID,
    /// If notifications of this type are ignored.
    pub ignore: bool,
    /// If notifications of this type are muted.
    pub mute: bool,
}

/// Notification type preferences properties of the Flexible Project system.
#[derive(Debug, InputObject, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NotificationTypePreferencesInput {
    /// Identifier of notification type.
    pub type_id: ID,
    /// If notifications of this type are ignored.
    pub ignore: bool,
    /// If notifications of this type are muted.
    pub mute: bool,
}
