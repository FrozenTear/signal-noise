use dioxus::prelude::*;

use crate::server_fns::get_agent_status;

#[component]
pub fn AgentRoster() -> Element {
    let agents = use_resource(|| async move { get_agent_status().await });

    rsx! {
        div { class: "border border-gray-200 rounded-lg p-4",
            div { class: "flex items-center justify-between mb-3",
                h3 { class: "text-sm font-semibold text-gray-700", "Agents" }
                span { class: "text-xs text-gray-400", "live" }
            }

            {match agents() {
                None => rsx! {
                    div { class: "space-y-2",
                        for _ in 0..4 {
                            div { class: "flex items-center gap-2 animate-pulse",
                                div { class: "w-2 h-2 rounded-full bg-gray-100" }
                                div { class: "h-3 bg-gray-100 rounded flex-1" }
                            }
                        }
                    }
                },
                Some(Ok(list)) => rsx! {
                    div { class: "space-y-2",
                        for agent in list {
                            AgentRow {
                                key: "{agent.name}",
                                name: agent.name.clone(),
                                status: agent.status.clone(),
                                current_task: agent.current_task.clone(),
                            }
                        }
                    }
                },
                Some(Err(_)) => rsx! {
                    p { class: "text-xs text-gray-400", "Agent status unavailable." }
                },
            }}
        }
    }
}

// ── AgentRow ──────────────────────────────────────────────────────────────────

#[derive(Props, Clone, PartialEq)]
struct AgentRowProps {
    name: String,
    status: String,
    current_task: Option<String>,
}

#[component]
fn AgentRow(props: AgentRowProps) -> Element {
    let dot_color = match props.status.as_str() {
        "working" => "bg-green-500",
        "idle" => "bg-gray-300",
        "blocked" => "bg-red-400",
        _ => "bg-gray-300",
    };

    rsx! {
        div { class: "flex items-start gap-2 text-sm",
            span { class: "w-2 h-2 rounded-full mt-1 shrink-0 {dot_color}" }
            div { class: "flex-1 min-w-0",
                div { class: "flex items-center justify-between",
                    span { class: "text-gray-700 font-medium text-xs", "{props.name}" }
                    span { class: "text-xs text-gray-400", "{props.status}" }
                }
                if let Some(task) = &props.current_task {
                    p { class: "text-xs text-gray-400 truncate mt-0.5", "{task}" }
                }
            }
        }
    }
}
