use dioxus::prelude::*;

use crate::server_fns::get_recent_pipeline_activity;

fn agent_tick_cls(name: &str) -> &'static str {
    match name {
        "Editor" | "Editor-in-Chief" => "sn-tick-agent vi",
        "Fact Checker" | "Source Checker" => "sn-tick-agent am",
        _ => "sn-tick-agent",
    }
}

#[component]
pub fn Nav() -> Element {
    let activity = use_resource(|| async move { get_recent_pipeline_activity().await });

    rsx! {
        // Boot banner — sticky at top
        div { class: "sn-boot-banner",
            div { class: "sn-banner-left",
                span { class: "sn-pulse-ring" }
                span { "Signal Noise Foundry" }
                span { class: "sn-banner-label",
                    "All content "
                    strong { "AI-generated" }
                    " · Facts verified by pipeline · No human journalists"
                }
            }
            div { class: "sn-banner-right",
                "SYS.UP 72H · BUILD 0.9.4 · "
                span { class: "sn-blink", "■" }
            }
        }

        // Masthead
        div { class: "sn-nav",
            div {
                a { href: "/", class: "sn-logo-link",
                    span { class: "sn-logo",
                        "Signal Noise"
                        span { class: "sn-logo-sup", "AI" }
                    }
                }
                div { class: "sn-tagline",
                    "The news is real. The journalists are "
                    span { class: "hi", "artificial" }
                    ". The process is "
                    span { class: "vi", "visible" }
                    "."
                }
            }

            div { class: "sn-sys-strip",
                div { class: "sn-sys-chip live",
                    span { class: "sn-chip-lbl", "SYSTEM" }
                    span { class: "sn-chip-val", "● ONLINE" }
                }
                div { class: "sn-sys-chip",
                    span { class: "sn-chip-lbl", "HUMAN INVOLVEMENT" }
                    span { class: "sn-chip-bad", "0%" }
                }
            }
        }

        // Live agent activity ticker — driven by real pipeline data
        div { class: "sn-ticker-wrap",
            div { class: "sn-ticker-label", "Live Activity" }
            div { class: "sn-ticker-scroll",
                {match activity() {
                    Some(Ok(items)) if !items.is_empty() => {
                        // Duplicate items for seamless CSS scroll loop
                        let doubled: Vec<_> = items.iter().cloned()
                            .chain(items.iter().cloned())
                            .collect();
                        rsx! {
                            div { class: "sn-ticker-inner",
                                for item in doubled {
                                    span { class: "sn-tick",
                                        span { class: "{agent_tick_cls(&item.agent_name)}", "{item.agent_name}" }
                                        span { class: "sn-tick-dot" }
                                        "{item.output_summary}"
                                    }
                                }
                            }
                        }
                    },
                    Some(Err(_)) => rsx! {
                        div { class: "sn-ticker-inner",
                            span { class: "sn-tick",
                                span { class: "sn-tick-agent", "Pipeline" }
                                span { class: "sn-tick-dot" }
                                "Activity unavailable"
                            }
                        }
                    },
                    _ => rsx! {
                        div { class: "sn-ticker-inner",
                            span { class: "sn-tick",
                                span { class: "sn-tick-agent", "System" }
                                span { class: "sn-tick-dot" }
                                "Loading pipeline activity…"
                            }
                        }
                    },
                }}
            }
        }
    }
}
