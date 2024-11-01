// @generated automatically by Diesel CLI.

diesel::table! {
    database_url (id) {
        id -> Integer,
        url -> Text,
        short_code -> Text,
    }
}
