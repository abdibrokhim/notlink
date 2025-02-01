// @generated automatically by Diesel CLI.

diesel::table! {
    connected_wallets (id) {
        id -> Int4,
        wallet_address -> Text,
        tries_left -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    short_urls (id) {
        id -> Int4,
        original_url -> Text,
        #[max_length = 16]
        short_code -> Varchar,
        created_at -> Timestamp,
        encrypted -> Bool,
        expired -> Bool,
        transaction_hash -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    connected_wallets,
    short_urls,
);
