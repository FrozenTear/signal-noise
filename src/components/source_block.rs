use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct SourceItem {
    pub url: String,
    pub name: String,
    pub source_type: String,
    pub paywall: bool,
    pub verified: bool,
}

#[derive(Props, Clone, PartialEq)]
pub struct SourceBlockProps {
    pub sources: Vec<SourceItem>,
}

#[component]
pub fn SourceBlock(props: SourceBlockProps) -> Element {
    rsx! {
        div { style: "margin-top:32px;",
            div { class: "sn-section-hdr",
                span { class: "hi", "SOURCES" }
            }
            div { class: "sn-sb-card",
                if props.sources.is_empty() {
                    div { style: "padding:14px 16px; font-family:var(--sn-mono); font-size:10px; color:var(--sn-text-dimmer);",
                        "No sources recorded."
                    }
                } else {
                    for source in &props.sources {
                        div { class: "sn-source-item",
                            div {
                                class: if source.verified { "sn-source-dot ok" } else { "sn-source-dot no" }
                            }
                            div {
                                a { style: "color:var(--sn-accent); text-decoration:none;",
                                    href: "{source.url}",
                                    "{source.name}"
                                }
                                span { style: "color:var(--sn-text-dimmer); margin-left:6px;",
                                    "[{source.source_type}]"
                                }
                                if source.paywall {
                                    span { class: "sn-chip-warn", style: "margin-left:6px;", "paywall" }
                                }
                                if source.verified {
                                    span { class: "sn-chip-val", style: "margin-left:6px;", "verified" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
