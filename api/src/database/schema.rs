// @generated automatically by Diesel CLI.

diesel::table! {
    availability (id) {
        id -> Int4,
        basic_event -> Text,
        name -> Text,
        availabilities -> Bytea,
    }
}

diesel::table! {
    basic_event (id) {
        id -> Text,
        name -> Text,
        when -> Bytea,
        no_ealier -> Time,
        no_later -> Time,
        timezone -> Text,
        created -> Timestamptz,
    }
}

diesel::joinable!(availability -> basic_event (basic_event));

diesel::allow_tables_to_appear_in_same_query!(availability, basic_event,);
