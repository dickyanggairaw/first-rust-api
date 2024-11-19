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

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 100]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        role_id -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(users -> role (role_id));

diesel::allow_tables_to_appear_in_same_query!(
    role,
    users,
);
