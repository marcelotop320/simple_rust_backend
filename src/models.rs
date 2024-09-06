use crate::schema::*;
use serde::Deserialize;

#[derive(Queryable, Insertable, Deserialize)]
#[table_name = "stations"]
pub struct Station {
    pub station_id: String,
    pub name: String,
    pub address: String,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Queryable, Insertable, Deserialize)]
#[table_name = "station_status"]
pub struct StationStatus {
    pub station_id: String,
    pub is_returning: bool,
    pub is_renting: bool,
    pub is_installed: bool,
    pub num_docks_available: i32,
    pub num_bikes_available: i32,
    pub last_reported: String,
}
