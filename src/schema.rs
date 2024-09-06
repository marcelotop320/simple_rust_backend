table! {
    stations (station_id) {
        station_id -> Varchar,
        name -> Varchar,
        address -> Varchar,
        latitude -> Float8,
        longitude -> Float8,
    }
}

table! {
    station_status (station_id) {
        station_id -> Varchar,
        is_returning -> Bool,
        is_renting -> Bool,
        is_installed -> Bool,
        num_docks_available -> Int4,
        num_bikes_available -> Int4,
        last_reported -> Timestamp,
    }
}
