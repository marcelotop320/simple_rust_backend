use rocket::serde::{json::Json, Deserialize};
use reqwest::get;
use crate::db::DbConn;
use crate::models::{Station, StationStatus};

#[derive(Deserialize)]
pub struct IngestRequest {
    gbfs_url: String,
}

#[post("/", format = "application/json", data = "<ingest_request>")]
pub async fn ingest_data(conn: DbConn, ingest_request: Json<IngestRequest>) -> &'static str {
    let gbfs_url = &ingest_request.gbfs_url;
    
    // Fetch GBFS data
    let station_info_url = format!("{}/station_information.json", gbfs_url);
    let station_status_url = format!("{}/station_status.json", gbfs_url);
    
    let station_info = get(station_info_url).await.unwrap().json::<Station>().await.unwrap();
    let station_status = get(station_status_url).await.unwrap().json::<StationStatus>().await.unwrap();
    
    // Update DB (logic to save data should be in models)
    Station::update_or_insert(&conn, &station_info).await;
    StationStatus::update_or_insert(&conn, &station_status).await;

    "Ingestion complete"
}
