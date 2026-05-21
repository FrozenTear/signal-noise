use dioxus::prelude::*;

use crate::components::confidence_meter::ConfidenceMeter;
use crate::components::nav::Nav;
use crate::components::pipeline_trail::{PipelineStepSummary, PipelineTrail};
use crate::components::source_block::{SourceBlock, SourceItem};
use crate::server_fns::{get_h2h_by_slug, ArticleDetail};
use crate::util::simple_md_to_html;

/// `/h2h/:slug` — renders an editor's-note intro plus the paired reporter pieces.
/// Editor's note is full-width; the two pieces sit in a two-column grid on
/// desktop (≥ 960px) and stack on mobile. Implements the THE-87 H2H layout spec
/// (`docs/published/h2h-2/LAYOUT-SPEC.md`), P0 scope.
#[component]
pub fn H2H(slug: String) -> Element {
    let slug_copy = slug.clone();
    let bundle = use_resource(move || {
        let s = slug_copy.clone();
        async move { get_h2h_by_slug(s).await }
    });

    rsx! {
        Nav {}
        div { class: "sn-article-page",
            {match bundle() {
                None => rsx! {
                    div { class: "sn-skeleton",
                        div { class: "sn-skeleton-bar", style: "width:20%" }
                        div { class: "sn-skeleton-bar", style: "width:75%" }
                        div { class: "sn-skeleton-bar", style: "width:60%; margin-bottom:32px" }
                        div { class: "sn-skeleton-bar", style: "width:100%" }
                        div { class: "sn-skeleton-bar", style: "width:90%" }
                    }
                },
                Some(Ok(None)) => rsx! {
                    div { style: "padding:64px 0; text-align:center;",
                        h2 { style: "font-size:18px; color:var(--sn-text); margin-bottom:8px;",
                            "Head-to-head not found"
                        }
                        p { style: "font-size:13px; color:var(--sn-text-dim); margin-bottom:16px;",
                            "This head-to-head may have been removed or the URL is incorrect."
                        }
                        a { style: "font-family:var(--sn-mono); font-size:11px; color:var(--sn-accent);",
                            href: "/",
                            "← back to feed"
                        }
                    }
                },
                Some(Ok(Some(h2h))) => {
                    let intro = h2h.intro;
                    let rendered_intro = simple_md_to_html(&intro.body);
                    let total = h2h.pieces.len();
                    rsx! {
                        a { style: "display:inline-flex; align-items:center; gap:6px; font-family:var(--sn-serif); font-size:14px; color:var(--sn-text-dimmer); text-decoration:none; margin-bottom:24px;",
                            href: "/",
                            "← Signal Noise"
                        }

                        // Editor's-note header + body (full width, centered column)
                        div { class: "sn-h2h-intro",
                            div { style: "display:flex; align-items:center; gap:10px; margin-bottom:14px;",
                                span { class: "sn-beat-tag {intro.category.to_lowercase()}", "{intro.category}" }
                                span { class: "sn-ts", "HEAD-TO-HEAD · {total} pieces" }
                            }
                            h1 { class: "sn-headline", style: "font-size:36px; margin-bottom:16px;",
                                "{intro.title}"
                            }
                            div { style: "display:flex; align-items:center; gap:12px; margin-bottom:8px;",
                                span { class: "sn-ts", "by {intro.persona_name}" }
                            }
                            ConfidenceMeter { score: intro.confidence_score }
                            div { class: "sn-disclaimer",
                                "ⓘ SYNTHETIC CONTENT — written by AI agents. All claims fact-checked by a separate AI process."
                            }
                            div { class: "prose", style: "margin:28px 0;",
                                dangerous_inner_html: "{rendered_intro}"
                            }
                        }

                        // Transparency divider — the primitive IS the design
                        div { class: "sn-h2h-divider",
                            "─── HEAD-TO-HEAD · NO COORDINATION · INDEPENDENT SOURCES ───"
                        }

                        // Paired pieces — two-column on desktop, stacked on mobile
                        div { class: "sn-h2h-grid",
                            for (idx, piece) in h2h.pieces.into_iter().enumerate() {
                                H2HColumn { key: "{piece.slug}", anchor: idx, piece }
                            }
                        }

                        // Mobile jump pill
                        if total >= 2 {
                            div { class: "sn-h2h-jump-pill",
                                a { href: "#h2h-piece-0", style: "color:var(--sn-accent); text-decoration:none;", "▲ first" }
                                span { style: "color:var(--sn-text-dimmer); margin:0 8px;", "·" }
                                a { href: "#h2h-piece-1", style: "color:var(--sn-violet); text-decoration:none;", "▼ second" }
                            }
                        }
                    }
                },
                Some(Err(_)) => rsx! {
                    p { style: "color:var(--sn-red); font-family:var(--sn-mono); font-size:12px; padding:16px 0;",
                        "ERROR: failed to load head-to-head."
                    }
                },
            }}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct H2HColumnProps {
    anchor: usize,
    piece: ArticleDetail,
}

#[component]
fn H2HColumn(props: H2HColumnProps) -> Element {
    let piece = props.piece;
    let rendered_body = simple_md_to_html(&piece.body);

    rsx! {
        article { id: "h2h-piece-{props.anchor}", class: "sn-h2h-col",
            div { style: "display:flex; align-items:center; gap:10px; margin-bottom:12px;",
                span { class: "sn-beat-tag {piece.category.to_lowercase()}", "{piece.category}" }
                span { class: "sn-ts", "{piece.persona_name}" }
            }
            h2 { class: "sn-headline", style: "font-size:24px; margin-bottom:12px;",
                "{piece.title}"
            }
            ConfidenceMeter { score: piece.confidence_score }
            div { class: "prose", style: "margin:20px 0;",
                dangerous_inner_html: "{rendered_body}"
            }

            if let Some(monologue) = &piece.ai_monologue {
                Collapsible { label: "AI monologue".to_string(), text: monologue.clone() }
            }
            if let Some(extended) = &piece.ai_monologue_extended {
                Collapsible { label: "Extended process log".to_string(), text: extended.clone() }
            }

            if !piece.sources.is_empty() {
                SourceBlock {
                    sources: piece.sources.iter().map(|s| SourceItem {
                        url: s.url.clone(),
                        name: s.name.clone(),
                        source_type: s.source_type.clone(),
                        paywall: s.paywall,
                        verified: s.verified,
                    }).collect()
                }
            }

            if !piece.pipeline.is_empty() {
                PipelineTrail {
                    steps: piece.pipeline.iter().map(|p| PipelineStepSummary {
                        agent_name: p.agent_name.clone(),
                        step_type: p.step_type.clone(),
                        output_summary: p.output_summary.clone(),
                        confidence_delta: p.confidence_delta,
                        completed_at: p.completed_at.clone(),
                        sort_order: p.sort_order,
                    }).collect()
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct CollapsibleProps {
    label: String,
    text: String,
}

#[component]
fn Collapsible(props: CollapsibleProps) -> Element {
    let mut open = use_signal(|| false);
    let rendered = simple_md_to_html(&props.text);
    let label = props.label.clone();

    rsx! {
        div { style: "margin:16px 0;",
            button {
                class: "sn-toggle-btn",
                onclick: move |_| open.toggle(),
                span { style: "color:var(--sn-violet);", "◈" }
                span { if open() { "hide {label}" } else { "show {label}" } }
            }
            if open() {
                div { class: "sn-monologue",
                    div { dangerous_inner_html: "{rendered}" }
                }
            }
        }
    }
}
