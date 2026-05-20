use dioxus::prelude::*;

use crate::components::confidence_meter::ConfidenceMeter;
use crate::components::nav::Nav;
use crate::components::pipeline_trail::{PipelineStepSummary, PipelineTrail};
use crate::components::source_block::{SourceBlock, SourceItem};
use crate::server_fns::{get_h2h_by_slug, ArticleDetail};
use crate::util::simple_md_to_html;

#[component]
pub fn Head2Head(slug: String) -> Element {
    let slug_copy = slug.clone();
    let bundle = use_resource(move || {
        let s = slug_copy.clone();
        async move { get_h2h_by_slug(s).await }
    });

    rsx! {
        Nav {}
        div { class: "sn-h2h-page",
            {match bundle() {
                None => rsx! {
                    div { class: "sn-skeleton",
                        div { class: "sn-skeleton-bar", style: "width:60%" }
                        div { class: "sn-skeleton-bar", style: "width:90%; margin-top:16px" }
                        div { class: "sn-skeleton-bar", style: "width:100%" }
                    }
                },
                Some(Ok(None)) => rsx! {
                    div { style: "padding:64px 0; text-align:center;",
                        h2 { style: "font-size:18px; color:var(--sn-text); margin-bottom:8px;",
                            "Head-to-head not found"
                        }
                        a { style: "font-family:var(--sn-mono); font-size:11px; color:var(--sn-accent);",
                            href: "/",
                            "← back to feed"
                        }
                    }
                },
                Some(Ok(Some(b))) => rsx! {
                    a { class: "sn-h2h-back",
                        href: "/",
                        "← Signal Noise"
                    }

                    // ── Editor's note (full-width, max 720px) ────────────────
                    IntroBlock { intro: b.intro.clone() }

                    // ── Divider strip ────────────────────────────────────────
                    div { class: "sn-h2h-divider",
                        span { class: "sn-h2h-divider-line" }
                        span { class: "sn-h2h-divider-text",
                            "HEAD-TO-HEAD · NO COORDINATION · INDEPENDENT SOURCES"
                        }
                        span { class: "sn-h2h-divider-line" }
                    }

                    // ── 2-column grid (lg:grid-cols-2 grid-cols-1) ───────────
                    div { class: "sn-h2h-grid",
                        for piece in b.pieces.iter() {
                            PieceColumn {
                                key: "{piece.slug}",
                                piece: piece.clone(),
                            }
                        }
                    }

                    // ── Mobile jump pill (hidden ≥ 1024px via CSS) ──────────
                    if b.pieces.len() >= 2 {
                        div { class: "sn-h2h-jump-pill",
                            a { href: "#piece-{b.pieces[0].slug}", "▲ Bolt" }
                            span { class: "sn-h2h-jump-sep", "·" }
                            a { href: "#piece-{b.pieces[1].slug}", "▼ Spark" }
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

// ── Editor's note block ──────────────────────────────────────────────────────

#[derive(Props, Clone, PartialEq)]
struct IntroBlockProps {
    intro: ArticleDetail,
}

#[component]
fn IntroBlock(props: IntroBlockProps) -> Element {
    let intro = props.intro;
    let body_html = simple_md_to_html(&intro.body);
    let byline = intro.byline.clone().unwrap_or_else(|| intro.persona_name.clone());

    rsx! {
        section { class: "sn-h2h-intro",
            div { class: "sn-h2h-intro-kicker",
                span { class: "sn-beat-tag {intro.category.to_lowercase()}", "{intro.category}" }
                span { class: "sn-h2h-intro-kicker-sep", "·" }
                span { "HEAD-TO-HEAD" }
            }
            h1 { class: "sn-h2h-intro-title", "{intro.title}" }
            div { class: "sn-h2h-intro-byline",
                "by {byline}"
                span { class: "sn-h2h-intro-byline-sep", "·" }
                span { "{intro.published_at}" }
            }
            ConfidenceMeter { score: intro.confidence_score }
            div { class: "prose sn-h2h-intro-body",
                dangerous_inner_html: "{body_html}"
            }
        }
    }
}

// ── Reporter piece column ────────────────────────────────────────────────────

#[derive(Props, Clone, PartialEq)]
struct PieceColumnProps {
    piece: ArticleDetail,
}

#[component]
fn PieceColumn(props: PieceColumnProps) -> Element {
    let p = props.piece;
    let body_html = simple_md_to_html(&p.body);
    let byline = p.byline.clone().unwrap_or_else(|| p.persona_name.clone());
    let model = p.model_attribution.clone().unwrap_or_default();
    let summary = p.summary.clone();

    rsx! {
        article {
            id: "piece-{p.slug}",
            class: "sn-h2h-col",

            // Reporter strip
            div { class: "sn-h2h-col-strip",
                div { class: "sn-h2h-col-byline",
                    span { class: "sn-h2h-col-by", "by " }
                    span { class: "sn-h2h-col-name", "{byline}" }
                }
                if !model.is_empty() {
                    span { class: "sn-h2h-col-model", "{model}" }
                }
            }

            // Beat tag
            div { class: "sn-h2h-col-beat",
                span { class: "sn-beat-tag {p.category.to_lowercase()}", "{p.category}" }
            }

            // Headline + summary
            h2 { class: "sn-headline sn-h2h-col-headline", "{p.title}" }
            if !summary.is_empty() {
                p { class: "sn-h2h-col-summary", "{summary}" }
            }

            // Confidence + body
            ConfidenceMeter { score: p.confidence_score }
            div { class: "prose sn-h2h-col-body",
                dangerous_inner_html: "{body_html}"
            }

            // AI monologue (short + extended)
            if let Some(text) = p.ai_monologue.clone() {
                AiMonologue { text }
            }
            if let Some(text) = p.ai_monologue_extended.clone() {
                AiMonologueExtended { text, persona_name: byline.clone() }
            }

            // Sources
            SourceBlock {
                sources: p.sources.iter().map(|s| SourceItem {
                    url: s.url.clone(),
                    name: s.name.clone(),
                    source_type: s.source_type.clone(),
                    paywall: s.paywall,
                    verified: s.verified,
                }).collect()
            }

            // Pipeline trail
            PipelineTrail {
                steps: p.pipeline.iter().map(|x| PipelineStepSummary {
                    agent_name: x.agent_name.clone(),
                    step_type: x.step_type.clone(),
                    output_summary: x.output_summary.clone(),
                    confidence_delta: x.confidence_delta,
                    completed_at: x.completed_at.clone(),
                    sort_order: x.sort_order,
                }).collect()
            }
        }
    }
}

// ── Collapsible AI monologue (mirrors article.rs primitives) ─────────────────

#[derive(Props, Clone, PartialEq)]
struct AiMonologueProps {
    text: String,
}

#[component]
fn AiMonologue(props: AiMonologueProps) -> Element {
    let mut open = use_signal(|| false);
    let rendered = simple_md_to_html(&props.text);

    rsx! {
        div { style: "margin:24px 0;",
            button {
                class: "sn-toggle-btn",
                onclick: move |_| open.toggle(),
                span { style: "color:var(--sn-violet);", "◈" }
                span { if open() { "hide AI monologue" } else { "show AI monologue" } }
            }
            if open() {
                div { class: "sn-monologue",
                    div { class: "sn-monologue-label", "INTERNAL REASONING" }
                    div { dangerous_inner_html: "{rendered}" }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct AiMonologueExtendedProps {
    text: String,
    persona_name: String,
}

#[component]
fn AiMonologueExtended(props: AiMonologueExtendedProps) -> Element {
    let mut open = use_signal(|| false);
    let rendered = simple_md_to_html(&props.text);

    rsx! {
        div { style: "margin:8px 0 24px;",
            button {
                class: "sn-toggle-btn",
                onclick: move |_| open.toggle(),
                span { style: "color:var(--sn-violet);", "◈" }
                span { if open() { "hide full process log" } else { "show full process log" } }
            }
            if open() {
                div { class: "sn-monologue sn-monologue-extended",
                    div { class: "sn-monologue-label", "EXTENDED INTERNAL MONOLOGUE · {props.persona_name}" }
                    div { dangerous_inner_html: "{rendered}" }
                }
            }
        }
    }
}
