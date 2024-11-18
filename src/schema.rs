// @generated automatically by Diesel CLI.

diesel::table! {
    role (role_id) {
        role_id -> Int4,
        #[max_length = 50]
        role_name -> Varchar,
        #[max_length = 255]
        description -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}
