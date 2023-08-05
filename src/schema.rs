// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "chat_type_enum"))]
    pub struct ChatTypeEnum;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "notification_type"))]
    pub struct NotificationType;
}

diesel::table! {
    auths (id) {
        access_token -> Text,
        refresh_token -> Text,
        created_at -> Nullable<Timestamptz>,
        is_valid -> Bool,
        expiration -> Nullable<Timestamptz>,
        auth_provider -> Text,
        user_id -> Nullable<Uuid>,
        id -> Uuid,
    }
}

diesel::table! {
    cats (id) {
        id -> Int4,
        name -> Text,
        age -> Int4,
        breed -> Text,
        color -> Text,
        weight -> Float8,
        image -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ChatTypeEnum;

    chat_rooms (id) {
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        last_message -> Nullable<Varchar>,
        chat_type -> ChatTypeEnum,
        id -> Uuid,
        last_sent_user_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    messages (id) {
        #[max_length = 255]
        message -> Varchar,
        sent_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
        id -> Uuid,
        chat_room_id -> Nullable<Uuid>,
        sent_by -> Nullable<Uuid>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::NotificationType;

    notifications (id) {
        id -> Uuid,
        user_id -> Nullable<Uuid>,
        user_to_notify -> Nullable<Uuid>,
        notification_type -> Nullable<NotificationType>,
        #[max_length = 255]
        data -> Nullable<Varchar>,
        read -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    user_chat_room (id) {
        user_id -> Nullable<Uuid>,
        chat_room_id -> Nullable<Uuid>,
        id -> Uuid,
    }
}

diesel::table! {
    users (id) {
        username -> Text,
        password -> Nullable<Text>,
        email -> Text,
        last_login_at -> Nullable<Timestamptz>,
        user_image -> Nullable<Text>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        tester -> Nullable<Bool>,
        id -> Uuid,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    auths,
    cats,
    chat_rooms,
    messages,
    notifications,
    user_chat_room,
    users,
);
