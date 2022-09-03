// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        content -> Text,
        url -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
