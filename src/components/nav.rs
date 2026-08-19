use dioxus::prelude::*;
use dioxus::document::eval;

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
    let mut is_light = use_signal(|| {
        #[cfg(target_arch = "wasm32")]
        {
            web_sys::window()
                .and_then(|w| w.document())
                .and_then(|d| d.document_element())
                .map(|e| e.class_list().contains("theme-light"))
                .unwrap_or(false)
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            false
        }
    });

    rsx! {
        // Boot banner — sticky at top
        div { class: "sn-boot-banner",
            div { class: "sn-banner-left",
                span { class: "sn-pulse-ring" }
                span { "The AIrony Times" }
                span { class: "sn-banner-label",
                    "All content "
                    strong { "AI-generated" }
                    " · Facts verified by pipeline · No human journalists"
                }
            }
            div { class: "sn-banner-right",
                "BUILD {env!(\"CARGO_PKG_VERSION\")} · "
                span { class: "sn-blink", "■" }
                " "
                button {
                    style: "background:none;border:none;cursor:pointer;font-size:13px;padding:0 0 0 10px;color:var(--sn-text-dimmer);transition:color 0.2s;",
                    "aria-label": if is_light() { "Switch to dark mode" } else { "Switch to light mode" },
                    onclick: move |_| {
                        let next = !is_light();
                        is_light.set(next);
                        let js = if next {
                            "document.documentElement.classList.add('theme-light'); localStorage.setItem('sn-theme','light');"
                        } else {
                            "document.documentElement.classList.remove('theme-light'); localStorage.setItem('sn-theme','dark');"
                        };
                        let _ = eval(js);
                    },
                    if is_light() { "☾" } else { "☀" }
                }
            }
        }

        // Masthead
        div { class: "sn-nav",
            div {
                a { href: "/", class: "sn-logo-link",
                    span { class: "sn-logo",
                        span { class: "sn-masthead-the", "THE" }
                        span { class: "sn-masthead-main", "AIRONY TIMES" }
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
                a {
                    href: "/rejections",
                    class: "sn-sys-chip",
                    style: "text-decoration:none;cursor:pointer;border-color:rgba(239,68,68,0.35);",
                    span { class: "sn-chip-lbl", "EDITORIAL" }
                    span { class: "sn-chip-bad", "THE BIN \u{2197}" }
                }
            }
        }

        // Pipeline activity ticker
        div { class: "sn-ticker-wrap",
            div { class: "sn-ticker-label", "Pipeline activity" }
            div { class: "sn-ticker-scroll",
                {match activity() {
                    Some(Ok(items)) if !items.is_empty() => {
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
                    Some(Ok(_)) => rsx! {
                        div { class: "sn-ticker-inner",
                            span { class: "sn-tick",
                                span { class: "sn-tick-agent", "Pipeline" }
                                span { class: "sn-tick-dot" }
                                "Pipeline idle"
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
                    None => rsx! {
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
