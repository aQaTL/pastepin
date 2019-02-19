table! {
    pastes (id) {
        id -> Varchar,
        filename -> Nullable<Varchar>,
        content -> Nullable<Text>,
        creation_date -> Timestamp,
    }
}
