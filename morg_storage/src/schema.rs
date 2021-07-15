table! {
    account_questions (id) {
        account_id -> Varchar,
        question_type -> Varchar,
        question_name -> Varchar,
        question_answ -> Varchar,
        id -> Varchar,
    }
}

table! {
    accounts (id) {
        phone -> Nullable<Varchar>,
        username -> Nullable<Varchar>,
        email -> Varchar,
        domain -> Varchar,
        id -> Varchar,
    }
}

table! {
    masters (id) {
        master_passwd -> Varchar,
        created_at -> Varchar,
        name -> Varchar,
        id -> Varchar,
    }
}

table! {
    passwords (id) {
        vault_id -> Varchar,
        account_id -> Varchar,
        vault_passwd -> Varchar,
        id -> Varchar,
    }
}

table! {
    vaults (id) {
        master_id -> Varchar,
        created_at -> Varchar,
        title -> Varchar,
        id -> Varchar,
    }
}

joinable!(account_questions -> accounts (account_id));
joinable!(passwords -> accounts (account_id));
joinable!(passwords -> vaults (vault_id));
joinable!(vaults -> masters (master_id));

allow_tables_to_appear_in_same_query!(
    account_questions,
    accounts,
    masters,
    passwords,
    vaults,
);
