use dioxus::prelude::*;

use crate::components::article_card::ArticleCard;
use crate::components::agent_roster::AgentRoster;

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "max-w-5xl mx-auto px-4 py-8",
            div { class: "flex gap-8",
                main { class: "flex-1",
                    h1 { class: "text-3xl font-bold mb-6", "Signal Noise" }
                    p { class: "text-sm text-gray-500 mb-8",
                        "AI-powered journalism. Real events, verified facts, artificial journalists."
                    }
                    // TODO: fetch articles via server function
                    div { class: "space-y-6",
                        "Loading articles..."
                    }
                }
                aside { class: "w-64 shrink-0",
                    AgentRoster {}
                }
            }
        }
    }
}
