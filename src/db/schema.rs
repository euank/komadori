// generate this file's content with `diesel print-schema`

table! {
    github_accounts (id) {
        id -> Int4,
        user_id -> Int4,
        access_token -> Nullable<Text>,
        username -> Nullable<Text>,
    }
}

table! {
    groups (id) {
        id -> Int4,
        uuid -> Uuid,
        name -> Text,
        public -> Bool,
        description -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    local_accounts (id) {
        id -> Int4,
        user_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        uuid -> Uuid,
        username -> Text,
        role -> Nullable<Text>,
        email -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users_groups (user_id, group_id) {
        user_id -> Int4,
        group_id -> Int4,
        owner -> Bool,
    }
}

joinable!(github_accounts -> users (user_id));
joinable!(local_accounts -> users (user_id));
joinable!(users_groups -> groups (group_id));
joinable!(users_groups -> users (user_id));

allow_tables_to_appear_in_same_query!(
    github_accounts,
    groups,
    local_accounts,
    users,
    users_groups,
);
