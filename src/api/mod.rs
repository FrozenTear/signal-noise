pub mod auth;
pub mod db;
pub mod rate_limit;
pub mod routes;

use surrealdb::{engine::local::Db, Surreal};

#[derive(Clone)]
pub struct AppState {
    pub db: Surreal<Db>,
}
