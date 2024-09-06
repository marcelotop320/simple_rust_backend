#[macro_use] extern crate rocket;

mod db;
mod models;
mod schema;
mod routes;

use routes::stations::get_stations;
use routes::status::get_station_status;
use routes::ingest::ingest_data;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/stations", routes![get_stations])
        .mount("/stations/<station_id>/status", routes![get_station_status])
        .mount("/ingest", routes![ingest_data])
}
