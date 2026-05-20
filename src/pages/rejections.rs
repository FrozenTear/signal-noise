use dioxus::prelude::*;

use crate::components::nav::Nav;
use crate::server_fns::{get_rejected_articles, RejectedArticleSummary};

#[component]
pub fn Rejections() -> Element {
    let rejected = use_resource(|| async move { get_rejected_articles().await });

    rsx! {
        Nav {}

        div { class: "sn-layout", style: "grid-template-columns:1fr;",
            main {
                // Header
                div { style: "display:flex;align-items:center;gap:14px;margin-bottom:8px;flex-wrap:wrap;",
                    span { class: "sn-beat-tag",
                        style: "font-size:10px;padding:4px 12px;background:rgba(239,68,68,0.12);color:var(--sn-red);border-color:rgba(239,68,68,0.3);",
                        "rejected"
                    }
                    h1 { class: "sn-headline", style: "margin-bottom:0;", "The Bin" }
                }
                p { style: "font-size:15px;color:var(--sn-text-dim);margin-bottom:28px;line-height:1.6;max-width:64ch;",
                    "Stories the Editor-in-Chief killed before they reached the front page. "
                    "Every rejection comes with a reason. Transparency does not get prettier "
                    "than this."
                }

                div { class: "sn-section-hdr",
                    span { style: "color:var(--sn-red);", "■" }
                    " Editorial Rejections"
                }

                {match rejected() {
                    None => rsx! {
                        RejectionSkeleton {}
                        RejectionSkeleton {}
                    },
                    Some(Ok(list)) if list.is_empty() => rsx! {
                        EmptyBin {}
                    },
                    Some(Ok(list)) => rsx! {
                        for art in list {
                            RejectionCard {
                                key: "{art.slug}",
                                item: art.clone(),
                            }
                        }
                    },
                    Some(Err(_)) => rsx! {
                        div { style: "font-family:var(--sn-mono);font-size:12px;color:var(--sn-red);padding:16px 0;",
                            "Pipeline error — failed to load rejection wall."
                        }
                    },
                }}
            }
        }

        div { class: "sn-footer",
            span { class: "hi", "SIGNAL NOISE" }
            " · Rejections are content too. We show our work, including the work we throw out."
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct RejectionCardProps {
    item: RejectedArticleSummary,
}

#[component]
fn RejectionCard(props: RejectionCardProps) -> Element {
    let item = &props.item;
    let acc_pct = (item.confidence_score * 100.0) as u32;
    let acc_cls = if item.confidence_score >= 0.80 {
        "g"
    } else if item.confidence_score >= 0.60 {
        "a"
    } else {
        "r"
    };
    let beat_cls = match item.category.as_str() {
        "linux"   => "linux",
        "privacy" => "privacy",
        _         => "tech",
    };

    let rejected_display = if item.rejected_at.len() >= 10 {
        item.rejected_at[..10].to_string()
    } else {
        item.rejected_at.clone()
    };

    let reason = item
        .rejection_reason
        .clone()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| "(no reason recorded)".to_string());

    rsx! {
        article {
            class: "sn-article beat-{beat_cls}",
            style: "border-left:3px solid var(--sn-red);opacity:0.96;",

            div { class: "sn-article-inner",
                // Meta row
                div { style: "display:flex;align-items:center;gap:10px;margin-bottom:14px;flex-wrap:wrap;",
                    span { class: "sn-beat-tag {beat_cls}", "{item.category}" }
                    span {
                        style: "font-family:var(--sn-mono);font-size:9px;letter-spacing:1px;text-transform:uppercase;color:var(--sn-red);",
                        "killed · {rejected_display}"
                    }
                    span { style: "display:inline-flex;align-items:center;gap:5px;font-family:var(--sn-mono);font-size:9px;",
                        span { style: "color:var(--sn-text-dimmer);", "confidence at rejection" }
                        span { class: "sn-conf-val {acc_cls}", "{acc_pct}%" }
                    }
                    span { style: "margin-left:auto;font-family:var(--sn-mono);font-size:9px;color:var(--sn-text-dimmer);",
                        "by {item.persona_name}"
                    }
                }

                // Title — struck through to read as killed
                h2 { class: "sn-headline",
                    style: "text-decoration:line-through;text-decoration-color:rgba(239,68,68,0.5);text-decoration-thickness:2px;",
                    "{item.title}"
                }

                // Summary
                if !item.summary.is_empty() {
                    p { class: "sn-summary", "{item.summary}" }
                }

                // The editorial verdict — the whole point of this page.
                div { class: "sn-monologue",
                    style: "border-left-color:var(--sn-red);background:rgba(239,68,68,0.06);",
                    div { class: "sn-monologue-label",
                        style: "color:var(--sn-red);",
                        "editorial verdict · Editor-in-Chief"
                    }
                    "{reason}"
                }
            }
        }
    }
}

#[component]
fn RejectionSkeleton() -> Element {
    rsx! {
        div { class: "sn-skeleton",
            div { class: "sn-skeleton-bar", style: "width:30%;margin-bottom:14px" }
            div { class: "sn-skeleton-bar", style: "width:85%;height:14px;margin-bottom:8px" }
            div { class: "sn-skeleton-bar", style: "width:70%;height:14px;margin-bottom:20px" }
            div { class: "sn-skeleton-bar", style: "width:100%" }
            div { class: "sn-skeleton-bar", style: "width:90%" }
        }
    }
}

#[component]
fn EmptyBin() -> Element {
    rsx! {
        div { style: "padding:48px 0;max-width:60ch;",
            div { style: "font-family:var(--sn-mono);font-size:11px;letter-spacing:2px;text-transform:uppercase;color:var(--sn-red);margin-bottom:14px;",
                "■ bin empty"
            }
            p { style: "font-size:17px;line-height:1.55;color:var(--sn-text);margin-bottom:14px;font-family:var(--sn-serif);font-style:italic;",
                "\u{201C}Nothing killed this week. Either the pipeline is excellent or the EIC is slacking.\u{201D}"
            }
            p { style: "font-size:13px;color:var(--sn-text-dimmer);font-family:var(--sn-mono);",
                "Rejections appear here the moment the Editor-in-Chief spikes a draft."
            }
        }
    }
}
