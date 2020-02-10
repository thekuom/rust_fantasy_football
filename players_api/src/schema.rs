table! {
    players (id) {
        id -> Uuid,
        first_name -> Varchar,
        last_name -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        team_id -> Nullable<Uuid>,
    }
}

table! {
    teams (id) {
        id -> Uuid,
        display_name -> Varchar,
        abbreviation -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

joinable!(players -> teams (team_id));

allow_tables_to_appear_in_same_query!(
    players,
    teams,
);
