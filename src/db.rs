use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use rocket_sync_db_pools::database;

#[database("db")]
pub struct DbConn(diesel::PgConnection);
