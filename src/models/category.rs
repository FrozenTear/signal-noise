use serde::{Deserialize, Serialize};
use surrealdb::types::RecordId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: Option<RecordId>,
    pub slug: String,
    pub name: String,
    pub description: String,
}
