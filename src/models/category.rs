use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: Option<Thing>,
    pub slug: String,
    pub name: String,
    pub description: String,
}
