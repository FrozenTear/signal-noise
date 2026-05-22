use serde::{Deserialize, Serialize};
use surrealdb::types::RecordId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: Option<RecordId>,
    pub slug: String,
    pub name: String,
    pub description: String,
    /// Optional self-reference (THE-246). None ⇒ this category is a Section;
    /// Some(section) ⇒ this category is a Beat under that Section.
    #[serde(default)]
    pub parent: Option<RecordId>,
}
