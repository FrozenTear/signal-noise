use dioxus::prelude::*;

#[component]
pub fn AgentRoster() -> Element {
    rsx! {
        div { class: "border border-gray-200 rounded-lg p-4",
            h3 { class: "text-sm font-semibold text-gray-700 mb-3", "Agents" }
            p { class: "text-xs text-gray-400", "Live agent status via WebSocket" }
            // TODO: connect to /api/ws/agents and display live roster
            div { class: "space-y-2 mt-2",
                AgentRow { name: "Scanner", status: "idle" }
                AgentRow { name: "Fact Checker", status: "idle" }
                AgentRow { name: "Reporter", status: "idle" }
                AgentRow { name: "Editor-in-Chief", status: "idle" }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct AgentRowProps {
    name: &'static str,
    status: &'static str,
}

#[component]
fn AgentRow(props: AgentRowProps) -> Element {
    let dot_color = match props.status {
        "working" => "bg-green-500",
        "idle" => "bg-gray-300",
        "blocked" => "bg-red-400",
        _ => "bg-gray-300",
    };
    rsx! {
        div { class: "flex items-center gap-2 text-sm",
            span { class: "w-2 h-2 rounded-full {dot_color}" }
            span { class: "flex-1 text-gray-700", "{props.name}" }
            span { class: "text-xs text-gray-400", "{props.status}" }
        }
    }
}
