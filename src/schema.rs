// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 255]
        full_name -> Varchar,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        is_admin -> Bool,
        is_active -> Bool,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}
