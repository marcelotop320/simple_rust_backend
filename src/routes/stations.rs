use rocket::serde::json::Json;
use crate::db::DbConn;
use crate::models::Station;

#[get("/")]
pub async fn get_stations(conn: DbConn) -> Json<Vec<Station>> {
    let stations = Station::all(&conn).await;
    Json(stations)
}
