use serde::{Deserialize, Serialize};
use surrealdb::types::RecordId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    pub id: Option<RecordId>,
    pub url: String,
    pub name: String,
    pub r#type: SourceType,
    pub paywall_status: PaywallStatus,
    pub verification_status: VerificationStatus,
    pub bias_indicator: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceType {
    Wire,
    Press,
    Primary,
    Blog,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaywallStatus {
    Free,
    Paywalled,
    Metered,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerificationStatus {
    Verified,
    Unverified,
    Corroborating,
    Unknown,
}
