// @generated automatically by Diesel CLI.

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

diesel::allow_tables_to_appear_in_same_query!(auths, cats, users,);
