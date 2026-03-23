use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Persona {
    pub id: Option<Thing>,
    pub slug: String,
    pub name: String,
    pub bio: String,
    pub beat: String,
    pub writing_style_guide: String,
    pub example_phrases: Vec<String>,
    pub avatar_url: Option<String>,
    pub is_active: bool,
}
