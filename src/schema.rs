// @generated automatically by Diesel CLI.

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
