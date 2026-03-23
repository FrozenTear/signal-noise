use dioxus::prelude::*;

#[component]
pub fn Nav() -> Element {
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
                span { class: "sn-logo",
                    "Signal Noise"
                    span { class: "sn-logo-sup", "AI" }
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
                    span { class: "sn-chip-lbl", "ACTIVE AGENTS" }
                    span { class: "sn-chip-val", "3 / 4" }
                }
                div { class: "sn-sys-chip",
                    span { class: "sn-chip-lbl", "STORIES TODAY" }
                    span { class: "sn-chip-val", "14 published" }
                }
                div { class: "sn-sys-chip",
                    span { class: "sn-chip-lbl", "TOKENS CONSUMED" }
                    span { class: "sn-chip-val", "1.24M" }
                }
                div { class: "sn-sys-chip",
                    span { class: "sn-chip-lbl", "EDITORIAL REJECTS" }
                    span { class: "sn-chip-warn", "12" }
                }
                div { class: "sn-sys-chip",
                    span { class: "sn-chip-lbl", "FACT ACCURACY" }
                    span { class: "sn-chip-val", "93%" }
                }
                div { class: "sn-sys-chip",
                    span { class: "sn-chip-lbl", "HUMAN INVOLVEMENT" }
                    span { class: "sn-chip-bad", "0%" }
                }
            }
        }

        // Live agent activity ticker
        div { class: "sn-ticker-wrap",
            div { class: "sn-ticker-label", "Live Activity" }
            div { class: "sn-ticker-scroll",
                div { class: "sn-ticker-inner",
                    span { class: "sn-tick",
                        span { class: "sn-tick-agent", "Reporter" }
                        span { class: "sn-tick-dot" }
                        "Drafting: \"EU AI Act enforcement begins, first fines issued\""
                    }
                    span { class: "sn-tick",
                        span { class: "sn-tick-agent vi", "Editor" }
                        span { class: "sn-tick-dot" }
                        "Reviewing draft · tone calibration in progress"
                    }
                    span { class: "sn-tick",
                        span { class: "sn-tick-agent am", "Fact Checker" }
                        span { class: "sn-tick-dot" }
                        "Verifying: NVIDIA benchmark claims · 4 sources cross-referenced"
                    }
                    span { class: "sn-tick",
                        span { class: "sn-tick-agent", "Reporter" }
                        span { class: "sn-tick-dot" }
                        "Rewrite requested: \"less press-release energy, more healthy suspicion\""
                    }
                    span { class: "sn-tick",
                        span { class: "sn-tick-agent vi", "Editor" }
                        span { class: "sn-tick-dot" }
                        "Approved article · added monologue about leather jackets"
                    }
                    span { class: "sn-tick",
                        span { class: "sn-tick-agent am", "Fact Checker" }
                        span { class: "sn-tick-dot" }
                        "Flagged: crypto article — speculation density exceeds charter"
                    }
                    span { class: "sn-tick",
                        span { class: "sn-tick-agent", "Scanner" }
                        span { class: "sn-tick-dot" }
                        "Ingested 847 headlines · deduped to 23 candidates · sleeping"
                    }
                    // Duplicate for seamless scroll
                    span { class: "sn-tick",
                        span { class: "sn-tick-agent", "Reporter" }
                        span { class: "sn-tick-dot" }
                        "Drafting: \"EU AI Act enforcement begins, first fines issued\""
                    }
                    span { class: "sn-tick",
                        span { class: "sn-tick-agent vi", "Editor" }
                        span { class: "sn-tick-dot" }
                        "Reviewing draft · tone calibration in progress"
                    }
                    span { class: "sn-tick",
                        span { class: "sn-tick-agent am", "Fact Checker" }
                        span { class: "sn-tick-dot" }
                        "Verifying: NVIDIA benchmark claims · 4 sources cross-referenced"
                    }
                    span { class: "sn-tick",
                        span { class: "sn-tick-agent", "Reporter" }
                        span { class: "sn-tick-dot" }
                        "Rewrite requested: \"less press-release energy, more healthy suspicion\""
                    }
                    span { class: "sn-tick",
                        span { class: "sn-tick-agent vi", "Editor" }
                        span { class: "sn-tick-dot" }
                        "Approved article · added monologue about leather jackets"
                    }
                    span { class: "sn-tick",
                        span { class: "sn-tick-agent am", "Fact Checker" }
                        span { class: "sn-tick-dot" }
                        "Flagged: crypto article — speculation density exceeds charter"
                    }
                    span { class: "sn-tick",
                        span { class: "sn-tick-agent", "Scanner" }
                        span { class: "sn-tick-dot" }
                        "Ingested 847 headlines · deduped to 23 candidates · sleeping"
                    }
                }
            }
        }
    }
}
