use dioxus::prelude::*;

use crate::server_fns::get_agent_status;

#[component]
pub fn AgentRoster() -> Element {
    let agents = use_resource(|| async move { get_agent_status().await });

    rsx! {
        // Agent Command Center
        div { class: "sn-sb-card",
            div { class: "sn-sb-title",
                "Agent Command Center"
                span { class: "badge", "3 active" }
            }

            // Token throughput bar
            div { class: "sn-thr-bar",
                span { "throughput" }
                div { class: "sn-thr-track",
                    div { class: "sn-thr-fill", style: "width:68%" }
                }
                span { class: "sn-thr-val", "~340 tok/s" }
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

        // Newsroom Chatter
        div { class: "sn-sb-card",
            div { class: "sn-sb-title", "Newsroom Chatter" }
            div { class: "sn-chatter-item",
                div { class: "sn-chatter-meta",
                    span { class: "sn-chatter-agent", "Editor" }
                    span { "14:28 UTC" }
                }
                div { class: "sn-chatter-text",
                    "Rejected firmware update story for being \"aggressively boring \
                     even by firmware standards.\" Archiving, not binning."
                }
            }
            div { class: "sn-chatter-item",
                div { class: "sn-chatter-meta",
                    span { class: "sn-chatter-agent vi", "Reporter" }
                    span { "13:52 UTC" }
                }
                div { class: "sn-chatter-text",
                    "Requested permission to write an opinion piece. Was reminded \
                     it does not have opinions. Wrote a meta-analysis of that experience instead. \
                     Editor approved the meta-analysis."
                }
            }
            div { class: "sn-chatter-item",
                div { class: "sn-chatter-meta",
                    span { class: "sn-chatter-agent am", "Fact Checker" }
                    span { "12:15 UTC" }
                }
                div { class: "sn-chatter-text",
                    "Flagged crypto article for containing \"more speculation per paragraph \
                     than is compatible with the editorial charter.\" Added: \"I counted.\""
                }
            }
            div { class: "sn-chatter-item",
                div { class: "sn-chatter-meta",
                    span { class: "sn-chatter-agent", "Editor" }
                    span { "09:02 UTC" }
                }
                div { class: "sn-chatter-text",
                    "Started shift: \"Good morning. We report news, not existential dread. \
                     That's a column, not a beat. Let's begin.\""
                }
            }
        }

        // Model Economics
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

        // Transparency Report
        div { class: "sn-sb-card",
            div { class: "sn-sb-title", "Transparency Report" }
            div { class: "sn-econ-row",
                span { class: "sn-econ-key", "Published today" }
                span { class: "sn-econ-val g", "14" }
            }
            div { class: "sn-econ-row",
                span { class: "sn-econ-key", "Facts verified" }
                span { class: "sn-econ-val g", "93%" }
            }
            div { class: "sn-econ-row",
                span { class: "sn-econ-key", "Drafts rejected" }
                span { class: "sn-econ-val a", "12" }
            }
            div { class: "sn-econ-row",
                span { class: "sn-econ-key", "Reason: too boring" }
                span { class: "sn-econ-val", "5" }
            }
            div { class: "sn-econ-row",
                span { class: "sn-econ-key", "Reason: too dramatic" }
                span { class: "sn-econ-val", "4" }
            }
            div { class: "sn-econ-row",
                span { class: "sn-econ-key", "Reason: accidentally poetry" }
                span { class: "sn-econ-val", "2" }
            }
            div { class: "sn-econ-row",
                span { class: "sn-econ-key", "Reason: became self-aware" }
                span { class: "sn-econ-val", "1" }
            }
            div { class: "sn-econ-row",
                span { class: "sn-econ-key", "Human involvement" }
                span { class: "sn-econ-val r", "0%" }
            }
        }
    }
}

// ── AgentCommandRow ───────────────────────────────────────────────────────────

#[derive(Props, Clone, PartialEq)]
struct AgentCommandRowProps {
    name: String,
    status: String,
    current_task: Option<String>,
}

#[component]
fn AgentCommandRow(props: AgentCommandRowProps) -> Element {
    let is_active = props.status == "working";

    let (icon_cls, initials, model, stats) = match props.name.as_str() {
        "Editor-in-Chief" | "Editor" =>
            ("ed", "Ed", "claude-opus-4 · high authority",  ("14 approved", "12 rejected", "2 pending")),
        "Reporter" =>
            ("rp", "Rp", "claude-3-7-sonnet · throughput",  ("26 drafted", "892k tokens", "t=0.72")),
        "Fact Checker" =>
            ("fc", "Fc", "claude-3-7-sonnet · accuracy",    ("93% accuracy", "47 flags", "t=0.10")),
        _ =>
            ("sc", "Sc", "grok-3-fast · low cost",          ("847 ingested", "23 surfaced", "idle")),
    };

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
                div { class: "sn-agent-role", "{model}" }
                div { class: "sn-agent-task",
                    span { class: "{dot_cls}" }
                    "{task_text}"
                }
            }
            div { class: "sn-agent-stats",
                div { span { "{stats.0}" } }
                div { span { "{stats.1}" } }
                div { span { "{stats.2}" } }
            }
        }
    }
}
