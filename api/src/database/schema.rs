// @generated automatically by Diesel CLI.

diesel::table! {
    basic_event (id) {
        id -> Text,
        name -> Text,
        when -> Text,
        no_ealier -> Time,
        no_later -> Time,
        timezone -> Text,
        created -> Timestamptz,
    }
}
