// @generated automatically by Diesel CLI.

// src/schema.rs
diesel::table! {
    users (id) {
        id -> Integer,
        name -> Text,
        email -> Text,
        created_at -> Timestamp,
    }
}
