use rocket::serde::json::Json;
use crate::db::DbConn;
use crate::models::StationStatus;

#[get("/<station_id>")]
pub async fn get_station_status(conn: DbConn, station_id: String) -> Json<Option<StationStatus>> {
    let status = StationStatus::get_by_station_id(&conn, &station_id).await;
    Json(status)
}
