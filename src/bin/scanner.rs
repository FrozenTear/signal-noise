use anyhow::Result;
use chrono::Utc;
use reqwest::Client;
use serde_json::json;
use std::env;
use std::collections::HashMap;

/// Type: (issue_title, source_urls)
/// Used to check for duplicate story candidates
type ExistingIssue = (String, Vec<String>);

/// Query Paperclip API for existing story candidate issues (last 7 days)
/// Returns: Vec<(issue_title, source_urls)>
async fn query_existing_candidates(
    client: &reqwest::Client,
    api_url: &str,
    company_id: &str,
    auth_header: &str,
) -> Result<Vec<ExistingIssue>, Box<dyn std::error::Error>> {
    let seven_days_ago = Utc::now() - chrono::Duration::days(7);

    tracing::info!("Querying existing story candidates from last 7 days...");

    // Query issues created after 7 days ago
    let url = format!(
        "{}/api/companies/{}/issues?status=todo,in_progress,in_review",
        api_url,
        company_id,
    );

    let response = client
        .get(&url)
        .header("Authorization", auth_header)
        .timeout(std::time::Duration::from_secs(30))
        .send()
        .await;

    match response {
        Ok(resp) => {
            match resp.json::<Vec<serde_json::Value>>().await {
                Ok(issues) => {
                    let mut existing = Vec::new();

                    for issue in issues {
                        if let Some(created_at_str) = issue.get("createdAt").and_then(|t| t.as_str()) {
                            if let Ok(created_at) = chrono::DateTime::parse_from_rfc3339(created_at_str) {
                                if created_at.with_timezone(&Utc) < seven_days_ago {
                                    continue; // Skip old issues
                                }
                            }
                        }

                        let title = issue
                            .get("title")
                            .and_then(|t| t.as_str())
                            .unwrap_or("")
                            .to_string();

                        // Extract URLs from description if present
                        let urls = if let Some(desc) = issue.get("description").and_then(|d| d.as_str()) {
                            // Simple extraction: look for https:// patterns
                            desc.split_whitespace()
                                .filter(|s| s.starts_with("https://") || s.starts_with("http://"))
                                .map(|s| s.to_string())
                                .collect()
                        } else {
                            Vec::new()
                        };

                        existing.push((title, urls));
                    }

                    tracing::info!("Found {} existing story candidate issues", existing.len());
                    Ok(existing)
                }
                Err(e) => {
                    tracing::warn!("Failed to parse Paperclip response: {}", e);
                    Ok(Vec::new()) // Graceful degradation
                }
            }
        }
        Err(e) => {
            tracing::warn!("Failed to query existing candidates: {}", e);
            Ok(Vec::new()) // Graceful degradation on API failure
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("signal_noise=debug".parse().unwrap()),
        )
        .init();

    tracing::info!("Scanner heartbeat starting...");

    // Get Paperclip environment variables
    let api_url = env::var("PAPERCLIP_API_URL")?;
    let api_key = env::var("PAPERCLIP_API_KEY")?;
    let company_id = env::var("PAPERCLIP_COMPANY_ID")?;
    let agent_id = env::var("PAPERCLIP_AGENT_ID")?;
    let run_id = env::var("PAPERCLIP_RUN_ID")?;

    // Create HTTP client with Paperclip auth
    let client = Client::new();
    let auth_header = format!("Bearer {}", api_key);

    tracing::info!("Checking inbox for existing assignments...");

    // Get my inbox
    let inbox_response = client
        .get(format!("{}/api/agents/me/inbox-lite", api_url))
        .header("Authorization", &auth_header)
        .send()
        .await?;

    let inbox: Vec<serde_json::Value> = inbox_response.json().await?;

    if !inbox.is_empty() {
        tracing::info!("Found {} assignments, working on those", inbox.len());
        return Ok(());
    }

    tracing::info!("No assignments in inbox. Starting RSS feed polling...");

    // Poll all feeds for story candidates
    let config_path = "config/feeds.toml";
    let candidates = signal_noise::scanner::poll_feeds(config_path).await?;
    tracing::info!("Found {} story candidates to process", candidates.len());

    // Create Paperclip issues for each story candidate
    let mut created_count = 0;
    for candidate in candidates {
        tracing::info!("Creating issue for: {}", candidate.headline);

        let mut issue_body = serde_json::Map::new();
        issue_body.insert(
            "title".to_string(),
            format!("[{}] {}", candidate.beat.to_uppercase(), candidate.headline).into(),
        );

        let mut description = candidate.summary.clone();
        description.push_str("\n\n## Story Candidate Details\n");
        description.push_str(&format!("- **Source Feed**: {}\n", candidate.source_feed));
        description.push_str(&format!(
            "- **Published**: {}\n",
            candidate
                .published_at
                .map(|d| d.to_rfc3339())
                .unwrap_or_else(|| "Unknown".to_string())
        ));
        description.push_str(&format!("- **Relevance Score**: {:.2}\n", candidate.relevance_score));
        description.push_str("- **Source URLs**:\n");
        for url in &candidate.source_urls {
            description.push_str(&format!("  - {}\n", url));
        }

        issue_body.insert("description".to_string(), description.into());
        issue_body.insert("status".to_string(), "todo".into());
        issue_body.insert(
            "priority".to_string(),
            match candidate.priority.as_str() {
                "high" => "high",
                "medium" => "medium",
                _ => "low",
            }
            .into(),
        );

        // Create the issue
        let issue_response = client
            .post(format!("{}/api/companies/{}/issues", api_url, company_id))
            .header("Authorization", &auth_header)
            .header("X-Paperclip-Run-Id", &run_id)
            .timeout(std::time::Duration::from_secs(10))
            .json(&serde_json::Value::Object(issue_body))
            .send()
            .await;

        match issue_response {
            Ok(resp) => {
                if resp.status().is_success() {
                    created_count += 1;
                    tracing::info!("✓ Created issue for story candidate");
                } else {
                    tracing::warn!("Failed to create issue: {}", resp.status());
                }
            }
            Err(e) => {
                tracing::warn!("Error creating issue: {}", e);
            }
        }
    }

    tracing::info!(
        "Scanner heartbeat complete. Created {} story candidate issues",
        created_count
    );
    Ok(())
}
