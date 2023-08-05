use crate::domains::notification::dto::notification_dto::Notification;
use crate::domains::notification::dto::notification_dto::NotificationTypeEnum;
use crate::schema::notifications;
use diesel::prelude::*;
use diesel::PgConnection;
use uuid::Uuid;

pub fn create_multiple_notifications(
    conn: &mut PgConnection,
    user_id: Uuid,
    users_to_notify: Vec<Uuid>,
    notification_type: NotificationTypeEnum,
    data: &str,
) -> Vec<Uuid> {
    // iterate over users_to_notify and create a notification for each user
    // return the notifications id
    let notifications = users_to_notify
        .iter()
        .map(|user_to_notify| {
            Notification::new(
                user_id,
                *user_to_notify,
                notification_type,
                data.to_string(),
            )
        })
        .collect::<Vec<Notification>>();

    // insert multiple notifications into the database with diesel
    diesel::insert_into(notifications::table)
        .values(&notifications)
        .execute(conn)
        .expect("Error inserting new notifications");
    // return the ids as Vec<Uuid>
    notifications
        .iter()
        .map(|notification| notification.id)
        .collect::<Vec<Uuid>>()
}

// pub fn read(conn: &mut PgConnection) -> Vec<Notification> {
//     // Implement your read logic here
// }

// pub fn read_one(conn: &mut PgConnection, id: i32) -> Option<Notification> {
//     // Implement your read_one logic here
// }

// pub fn update_one(conn: &mut PgConnection, id: i32, notification: &Notification) -> Notification {
//     // Implement your update_one logic here
// }

// pub fn delete_one(conn: &mut PgConnection, id: i32) -> usize {
//     // Implement your delete_one logic here
// }
