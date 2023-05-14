//! Notification data model of the gateway service.

#![allow(missing_docs)]

use async_graphql::{InputObject, Interface, Object, SimpleObject, Subscription, ID};
use chrono::{DateTime, Utc};
use futures::{stream::Repeat, Stream};

use super::{user::User, workspace::Workspace};

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
    pub r#type: NotificationType,
    /// Message of the notification.
    pub message: String,
    /// Source of the notification (could be another user, workspace or project).
    pub source: NotificationSource,
    /// Destination of the notification (always user).
    pub destination: User,
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

/// Source of the notification: either another user, workspace or project.
#[derive(Debug, Interface, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[graphql(field(name = "id", type = "&ID", desc = "Unique identifier of the object."))]
pub enum NotificationSource {
    /// Source of the notification is another user.
    User(User),
    /// Source of the notification is workspace.
    Workspace(Workspace),
    // TODO Project(Project),
}

/// Notification preferences properties of the Flexible Project system.
#[derive(Debug, SimpleObject, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NotificationPreferences {
    /// User which owns these preferences.
    pub user: User,
    /// Set of preferences for each notification type.
    pub type_preferences: Vec<NotificationTypePreferences>,
}

/// Data of notification preferences to update.
#[derive(Debug, InputObject)]
pub struct UpdateNotificationPreferences {
    /// Set of preferences for each notification type to update, if present.
    pub type_preferences: Option<Vec<UpdateNotificationTypePreferences>>,
}

/// Notification type preferences properties of the Flexible Project system.
#[derive(Debug, SimpleObject, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NotificationTypePreferences {
    /// Type of the notification.
    pub r#type: NotificationType,
    /// If notifications of this type are ignored.
    pub ignore: bool,
    /// If notifications of this type are muted.
    pub mute: bool,
}

/// Notification type preferences properties of the Flexible Project system.
#[derive(Debug, InputObject, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct UpdateNotificationTypePreferences {
    /// Identifier of notification type.
    pub type_id: ID,
    /// Updates if notifications of this type are ignored, if exists.
    pub ignore: Option<bool>,
    /// Updates if notifications of this type are muted, if exists.
    pub mute: Option<bool>,
}
