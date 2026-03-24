use dioxus::prelude::*;

use crate::server_fns::{get_agent_status, get_recent_pipeline_activity, get_transparency_stats};

#[component]
pub fn AgentRoster() -> Element {
    let agents   = use_resource(|| async move { get_agent_status().await });
    let stats    = use_resource(|| async move { get_transparency_stats().await });
    let activity = use_resource(|| async move { get_recent_pipeline_activity().await });

    // Derive active-agent count for badge
    let active_count = agents().and_then(|r| r.ok()).map(|list| {
        list.iter().filter(|a| a.status == "working").count()
    });

    rsx! {
        // Agent Command Center
        div { class: "sn-sb-card",
            div { class: "sn-sb-title",
                "Agent Command Center"
                {match active_count {
                    Some(n) if n > 0 => rsx! {
                        span { class: "badge", "{n} active" }
                    },
                    _ => rsx! {
                        span { class: "badge", "live" }
                    },
                }}
            }

            {match agents() {
                None => rsx! {
                    for _ in 0..4 {
                        div { class: "sn-agent-row",
                            div { class: "sn-agent-icon ed" }
                            div {
                                div { class: "sn-skeleton-bar", style: "width:70%;margin-bottom:6px" }
                                div { class: "sn-skeleton-bar", style: "width:50%" }
                            }
                            div {}
                        }
                    }
                },
                Some(Ok(list)) => rsx! {
                    for agent in list {
                        AgentCommandRow {
                            key: "{agent.name}",
                            name: agent.name.clone(),
                            model: agent.model.clone(),
                            status: agent.status.clone(),
                            current_task: agent.current_task.clone(),
                        }
                    }
                },
                Some(Err(_)) => rsx! {
                    div { style: "padding:12px 16px;font-family:var(--sn-mono);font-size:10px;color:var(--sn-text-dimmer);",
                        "Agent status unavailable."
                    }
                },
            }}
        }

        // Newsroom Chatter — live pipeline activity, falls back to mock
        div { class: "sn-sb-card",
            div { class: "sn-sb-title", "Newsroom Chatter" }
            {match activity() {
                None => rsx! {
                    div { class: "sn-chatter-item",
                        div { class: "sn-skeleton-bar", style: "width:80%;margin-bottom:6px" }
                        div { class: "sn-skeleton-bar", style: "width:60%" }
                    }
                },
                Some(Ok(items)) => rsx! {
                    for item in items {
                        ChatterItem {
                            key: "{item.completed_at}{item.agent_name}",
                            agent_name: item.agent_name.clone(),
                            text: item.output_summary.clone(),
                            timestamp: item.completed_at.clone(),
                        }
                    }
                },
                Some(Err(_)) => rsx! {
                    div { style: "padding:12px 16px;font-family:var(--sn-mono);font-size:10px;color:var(--sn-text-dimmer);",
                        "No recent activity."
                    }
                },
            }}
        }

        // Model Economics — instrumentation not yet available; shown as illustrative
        div { class: "sn-sb-card",
            div { class: "sn-sb-title", "Model Economics" }
            div { class: "sn-econ-row",
                span { class: "sn-econ-key", "Total API spend today" }
                span { class: "sn-econ-val a", "$0.84" }
            }
            div { class: "sn-econ-row",
                span { class: "sn-econ-key", "Tokens consumed" }
                span { class: "sn-econ-val", "1.24M" }
            }
            div { class: "sn-econ-row",
                span { class: "sn-econ-key", "Avg cost / article" }
                span { class: "sn-econ-val g", "$0.060" }
            }
            div { class: "sn-econ-row",
                span { class: "sn-econ-key", "Human journalist salary" }
                span { class: "sn-econ-val r", "$0" }
            }
        }

        // Transparency Report — real counts from SurrealDB
        div { class: "sn-sb-card",
            div { class: "sn-sb-title", "Transparency Report" }
            {match stats() {
                None => rsx! {
                    div { class: "sn-skeleton-bar", style: "width:70%;margin:8px 16px" }
                    div { class: "sn-skeleton-bar", style: "width:50%;margin:8px 16px" }
                },
                Some(Ok(s)) => rsx! {
                    div { class: "sn-econ-row",
                        span { class: "sn-econ-key", "Published today" }
                        span { class: "sn-econ-val g", "{s.published_today}" }
                    }
                    div { class: "sn-econ-row",
                        span { class: "sn-econ-key", "Published total" }
                        span { class: "sn-econ-val g", "{s.published_total}" }
                    }
                    div { class: "sn-econ-row",
                        span { class: "sn-econ-key", "Drafts rejected" }
                        span { class: "sn-econ-val a", "{s.rejected_total}" }
                    }
                    div { class: "sn-econ-row",
                        span { class: "sn-econ-key", "Human involvement" }
                        span { class: "sn-econ-val r", "0%" }
                    }
                },
                Some(Err(_)) => rsx! {
                    div { style: "padding:12px 16px;font-family:var(--sn-mono);font-size:10px;color:var(--sn-text-dimmer);",
                        "Stats unavailable."
                    }
                },
            }}
        }
    }
}

// ── ChatterItem ───────────────────────────────────────────────────────────────

#[derive(Props, Clone, PartialEq)]
struct ChatterItemProps {
    agent_name: String,
    text: String,
    timestamp: String,
}

#[component]
fn ChatterItem(props: ChatterItemProps) -> Element {
    let cls = match props.agent_name.as_str() {
        "Reporter"                             => "sn-chatter-agent vi",
        "Fact Checker" | "Source Checker"      => "sn-chatter-agent am",
        "Scanner"                              => "sn-chatter-agent sc",
        _                                      => "sn-chatter-agent",
    };
    rsx! {
        div { class: "sn-chatter-item",
            div { class: "sn-chatter-meta",
                span { class: "{cls}", "{props.agent_name}" }
                span { "{props.timestamp}" }
            }
            div { class: "sn-chatter-text", "{props.text}" }
        }
    }
}

// ── AgentCommandRow ───────────────────────────────────────────────────────────

#[derive(Props, Clone, PartialEq)]
struct AgentCommandRowProps {
    name: String,
    model: Option<String>,
    status: String,
    current_task: Option<String>,
}

#[component]
fn AgentCommandRow(props: AgentCommandRowProps) -> Element {
    let is_active = props.status == "working";

    let (icon_cls, initials) = match props.name.as_str() {
        "Editor-in-Chief" | "Editor" => ("ed", "Ed"),
        "Reporter"                   => ("rp", "Rp"),
        "Fact Checker" | "Source Checker" => ("fc", "Fc"),
        "Scanner"                    => ("sc", "Sc"),
        "Article Verifier"           => ("av", "Av"),
        _                            => ("sc", &props.name[..2.min(props.name.len())]),
    };

    let model_label = props.model
        .as_deref()
        .unwrap_or("claude-sonnet-4-6");

    let active_cls = if is_active { "sn-agent-icon active" } else { "sn-agent-icon" };
    let dot_cls    = if is_active { "sn-task-dot active"   } else { "sn-task-dot idle" };

    let task_text = props.current_task
        .as_deref()
        .unwrap_or("Idle — awaiting next task");

    rsx! {
        div { class: "sn-agent-row",
            div { class: "{active_cls} {icon_cls}", "{initials}" }
            div {
                div { class: "sn-agent-name", "{props.name}" }
                div { class: "sn-agent-role", "{model_label}" }
                div { class: "sn-agent-task",
                    span { class: "{dot_cls}" }
                    "{task_text}"
                }
            }
        }
    }
}
