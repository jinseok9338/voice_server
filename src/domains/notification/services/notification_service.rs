use diesel::pg::PgConnection;
use uuid::Uuid;

use crate::domains::notification::dto::notification_dto::NotificationTypeEnum;

use super::database::notification_database::create_multiple_notifications;

pub struct NotificationService<'a> {
    pub conn: &'a mut PgConnection,
}

impl<'a> NotificationService<'a> {
    pub fn new(conn: &'a mut PgConnection) -> Self {
        Self { conn }
    }

    pub fn create_notifications(
        &mut self,
        user_id: Uuid,
        users_to_notify: Vec<Uuid>,
        notification_type: NotificationTypeEnum,
        data: &str,
    ) -> Vec<Uuid> {
        // filter out the user_id in the users_to_notify
        let users_to_notify = users_to_notify
            .into_iter()
            .filter(|&x| x != user_id)
            .collect::<Vec<Uuid>>();
        
        create_multiple_notifications(self.conn, user_id, users_to_notify, notification_type, data)
    }

    // pub fn read_notifications(&mut self) -> Vec<Notification> {
    //     read(self.conn)
    // }

    // pub fn read_one_notification(&mut self, id: i32) -> Option<Notification> {
    //     read_one(self.conn, id)
    // }

    // pub fn update_notification(&mut self, notification: &Notification) -> Notification {
    //     update_one(self.conn, notification.id, notification)
    // }

    // pub fn delete_notification(&mut self, id: i32) -> usize {
    //     delete_one(self.conn, id)
    // }
}
