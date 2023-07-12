// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "chat_type_enum"))]
    pub struct ChatTypeEnum;
}

diesel::table! {
    auths (id) {
        id -> Int4,
        access_token -> Text,
        refresh_token -> Text,
        user_id -> Int4,
        created_at -> Nullable<Timestamptz>,
        is_valid -> Bool,
        expiration -> Nullable<Timestamptz>,
        auth_provider -> Text,
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
        id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        last_message -> Nullable<Varchar>,
        last_sent_user_id -> Nullable<Int4>,
        chat_type -> ChatTypeEnum,
    }
}

diesel::table! {
    messages (id) {
        id -> Int4,
        chat_room_id -> Int4,
        sent_by -> Int4,
        #[max_length = 255]
        message -> Varchar,
        sent_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    user_chat_room (user_id, chat_room_id) {
        user_id -> Int4,
        chat_room_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        password -> Nullable<Text>,
        email -> Text,
        last_login_at -> Nullable<Timestamptz>,
        user_image -> Nullable<Text>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        tester -> Nullable<Bool>,
    }
}

diesel::joinable!(auths -> users (user_id));
diesel::joinable!(chat_rooms -> users (last_sent_user_id));
diesel::joinable!(messages -> chat_rooms (chat_room_id));
diesel::joinable!(messages -> users (sent_by));
diesel::joinable!(user_chat_room -> chat_rooms (chat_room_id));
diesel::joinable!(user_chat_room -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    auths,
    cats,
    chat_rooms,
    messages,
    user_chat_room,
    users,
);
