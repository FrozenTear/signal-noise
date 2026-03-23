use anyhow::Result;
use chrono::Utc;
use reqwest::Client;
use std::env;

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
) -> Vec<ExistingIssue> {
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
                        // Check if issue is recent (Issue 4: Add logging for date parsing failures)
                        let is_recent = if let Some(created_at_str) = issue.get("createdAt").and_then(|t| t.as_str()) {
                            match chrono::DateTime::parse_from_rfc3339(created_at_str) {
                                Ok(created_at) => created_at.with_timezone(&Utc) >= seven_days_ago,
                                Err(e) => {
                                    tracing::debug!("Failed to parse createdAt: {} (value: {})", e, created_at_str);
                                    true // Include in results if parse fails (be inclusive)
                                }
                            }
                        } else {
                            tracing::debug!("Missing createdAt field");
                            true // Include issues without date (be inclusive)
                        };

                        if !is_recent {
                            continue;
                        }

                        // Issue 3: Skip issues with missing/empty titles
                        let title = match issue.get("title").and_then(|t| t.as_str()) {
                            Some(t) if !t.trim().is_empty() => t.to_string(),
                            _ => {
                                tracing::debug!("Skipping issue without title");
                                continue;
                            }
                        };

                        // Extract URLs from description if present (Issue 2: More robust URL extraction)
                        let urls = if let Some(desc) = issue.get("description").and_then(|d| d.as_str()) {
                            // Extract URLs more robustly: find http(s):// and collect until whitespace/punctuation
                            let mut extracted_urls = Vec::new();
                            let mut remaining = desc;

                            while let Some(start_idx) = remaining.find("http://").or_else(|| remaining.find("https://")) {
                                // Find the protocol start
                                let protocol_start = if remaining[start_idx..].starts_with("https://") { start_idx } else { start_idx };
                                let url_start = protocol_start;

                                // Find the end of the URL (whitespace, quotes, brackets, or punctuation)
                                let url_end = remaining[url_start..]
                                    .find(|c: char| c.is_whitespace() || c == ')' || c == ',' || c == '"' || c == '>' || c == ']')
                                    .map(|i| url_start + i)
                                    .unwrap_or(remaining.len());

                                let url = &remaining[url_start..url_end];
                                if !url.is_empty() {
                                    extracted_urls.push(url.to_string());
                                }

                                // Move past this URL for next iteration
                                remaining = &remaining[url_end..];
                            }

                            extracted_urls
                        } else {
                            Vec::new()
                        };

                        existing.push((title, urls));
                    }

                    // Issue 5: Add response size limit
                    const MAX_ISSUES: usize = 1000;
                    if existing.len() > MAX_ISSUES {
                        tracing::warn!("Received {} issues, truncating to {}", existing.len(), MAX_ISSUES);
                    }
                    let existing = &existing[..std::cmp::min(existing.len(), MAX_ISSUES)];

                    tracing::info!("Found {} existing story candidate issues", existing.len());
                    existing.to_vec()
                }
                Err(e) => {
                    tracing::warn!("Failed to parse Paperclip response: {}", e);
                    Vec::new() // Graceful degradation
                }
            }
        }
        Err(e) => {
            tracing::warn!("Failed to query existing candidates: {}", e);
            Vec::new() // Graceful degradation on API failure
        }
    }
}

/// Deduplicate story candidates against existing issues
/// Returns: candidates that are new (not duplicates)
fn filter_new_candidates(
    candidates: Vec<signal_noise::scanner::StoryCandidate>,
    existing: &[ExistingIssue],
) -> Vec<signal_noise::scanner::StoryCandidate> {
    let mut new_candidates = Vec::new();
    let mut skipped_count = 0;

    // Two-tier deduplication: O(n*m) complexity acceptable due to:
    // - 7-day lookback limit (~1000 max existing issues per line 107)
    // - Break statements prevent redundant checking
    // - Fast path (URL matching) exits before slower path (title similarity)
    for candidate in candidates {
        let mut is_duplicate = false;

        // Tier 1: Check URL matching (exact duplicates)
        for (_, existing_urls) in existing {
            if existing_urls.iter().any(|url| {
                candidate.source_urls.iter().any(|c_url| c_url == url)
            }) {
                is_duplicate = true;
                tracing::debug!("Skipped duplicate by URL: {}", candidate.headline);
                skipped_count += 1;
                break;
            }
        }

        // Tier 2: Check title similarity (if URL is new)
        if !is_duplicate {
            for (existing_title, _) in existing {
                let similarity = string_similarity(&candidate.headline, existing_title);
                // 0.85 similarity threshold catches story variants (same news from different outlets)
                // while avoiding false positives from unrelated stories. Tuned in SIG-113.
                if similarity > 0.85 {
                    is_duplicate = true;
                    tracing::debug!(
                        "Skipped similar story (similarity={:.2}): {}",
                        similarity,
                        candidate.headline
                    );
                    skipped_count += 1;
                    break;
                }
            }
        }

        if !is_duplicate {
            new_candidates.push(candidate);
        }
    }

    if skipped_count > 0 {
        tracing::info!("Deduplication: skipped {} duplicates", skipped_count);
    }

    new_candidates
}

/// Calculate string similarity using Levenshtein distance
/// Returns value between 0.0 and 1.0 (1.0 = identical)
/// Note: Strings longer than 1000 chars are truncated to prevent pathological performance
fn string_similarity(s1: &str, s2: &str) -> f64 {
    const MAX_LEN: usize = 1000;

    let s1 = if s1.len() > MAX_LEN { &s1[..MAX_LEN] } else { s1 };
    let s2 = if s2.len() > MAX_LEN { &s2[..MAX_LEN] } else { s2 };

    let max_len = std::cmp::max(s1.len(), s2.len());
    if max_len == 0 {
        return 1.0;
    }

    let distance = strsim::levenshtein(s1, s2);
    1.0 - (distance as f64 / max_len as f64)
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
    let _agent_id = env::var("PAPERCLIP_AGENT_ID")?;
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

    // Query existing issues for deduplication
    let existing_issues = query_existing_candidates(
        &client,
        &api_url,
        &company_id,
        &auth_header,
    )
    .await;

    // Filter out duplicates
    let candidates = filter_new_candidates(candidates, &existing_issues);
    tracing::info!("After deduplication: {} new candidates", candidates.len());

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
