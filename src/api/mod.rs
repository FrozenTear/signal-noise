pub mod db;
pub mod routes;

use surrealdb::{engine::local::Db, Surreal};

#[derive(Clone)]
pub struct AppState {
    pub db: Surreal<Db>,
}
