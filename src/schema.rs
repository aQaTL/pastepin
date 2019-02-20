table! {
    pastes (id) {
        id -> Int8,
        filename -> Nullable<Varchar>,
        content -> Nullable<Text>,
        creation_date -> Timestamp,
    }
}
