// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        name -> Text,
        email -> Text,
        created_at -> Nullable<Timestamptz>,
    }
}
