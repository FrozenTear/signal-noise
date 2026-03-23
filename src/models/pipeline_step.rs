use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::types::RecordId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineStep {
    pub id: Option<RecordId>,
    pub article: RecordId,
    pub agent_name: String,
    pub step_type: StepType,
    pub input_summary: String,
    pub output_summary: String,
    pub confidence_delta: f64,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StepType {
    Scan,
    FactCheck,
    Draft,
    Edit,
}
