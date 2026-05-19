use dioxus::prelude::*;

use crate::components::confidence_meter::ConfidenceMeter;
use crate::components::nav::Nav;
use crate::components::pipeline_trail::{PipelineStepSummary, PipelineTrail};
use crate::components::source_block::{SourceBlock, SourceItem};
use crate::components::transparency_hud::{HudPipelineStep, HudSource, TransparencyHud};
use crate::server_fns::get_article_by_slug;
use crate::util::simple_md_to_html;

#[component]
pub fn Article(slug: String) -> Element {
    let slug_copy = slug.clone();
    let article = use_resource(move || {
        let s = slug_copy.clone();
        async move { get_article_by_slug(s).await }
    });

    rsx! {
        Nav {}
        div { class: "sn-article-page",
            {match article() {
                None => rsx! {
                    div { class: "sn-skeleton",
                        div { class: "sn-skeleton-bar", style: "width:15%" }
                        div { class: "sn-skeleton-bar", style: "width:70%" }
                        div { class: "sn-skeleton-bar", style: "width:55%" }
                        div { class: "sn-skeleton-bar", style: "width:100%; margin-top:24px" }
                        div { class: "sn-skeleton-bar", style: "width:100%" }
                        div { class: "sn-skeleton-bar", style: "width:80%" }
                    }
                },
                Some(Ok(None)) => rsx! {
                    div { style: "padding:64px 0; text-align:center;",
                        h2 { style: "font-size:18px; color:var(--sn-text); margin-bottom:8px;",
                            "Article not found"
                        }
                        p { style: "font-size:13px; color:var(--sn-text-dim); margin-bottom:16px;",
                            "This article may have been removed or the URL is incorrect."
                        }
                        a { style: "font-family:var(--sn-mono); font-size:11px; color:var(--sn-accent);",
                            href: "/",
                            "← back to feed"
                        }
                    }
                },
                Some(Ok(Some(art))) => {
                    let rendered_body = simple_md_to_html(&art.body);
                    let hud_confidence = art.confidence_score;
                    let hud_sources: Vec<HudSource> = art.sources.iter().map(|s| HudSource {
                        name: s.name.clone(),
                        verified: s.verified,
                    }).collect();
                    let hud_steps: Vec<HudPipelineStep> = art.pipeline.iter().map(|p| HudPipelineStep {
                        step_type: p.step_type.clone(),
                        confidence_delta: p.confidence_delta,
                    }).collect();
                    let hud_monologue = art.ai_monologue.clone();
                    let hud_persona = Some(art.persona_name.clone());
                    rsx! {
                    // Back link
                    a { style: "display:inline-flex; align-items:center; gap:6px; font-family:var(--sn-serif); font-size:14px; color:var(--sn-text-dimmer); text-decoration:none; margin-bottom:24px; transition:color 0.2s;",
                        href: "/",
                        "← Signal Noise"
                    }

                    // Gen-bar (AI metadata strip)
                    div { class: "sn-gen-bar",
                        div { class: "sn-gen-pill model", "claude-sonnet-4-6" }
                        div { class: "sn-gen-pill tokens", "{art.persona_name}" }
                        div { class: "sn-gen-spacer" }
                        span { class: "sn-ts", "{art.published_at}" }
                    }

                    div { class: "sn-article-page-grid",
                        // ── Main column: focused reading ──────────────────────
                        div {
                            // Category + byline
                            div { style: "display:flex; align-items:center; gap:10px; margin-bottom:14px; margin-top:20px;",
                                span { class: "sn-beat-tag {art.category.to_lowercase()}", "{art.category}" }
                                span { class: "sn-ts", "by {art.persona_name}" }
                            }

                            // Title
                            h1 { class: "sn-headline", style: "font-size:36px; margin-bottom:16px;",
                                "{art.title}"
                            }

                            // Confidence meter
                            ConfidenceMeter { score: art.confidence_score }

                            // AI disclaimer banner
                            div { class: "sn-disclaimer",
                                "ⓘ SYNTHETIC CONTENT — written by AI agents. All claims fact-checked by a separate AI process."
                            }

                            // Article body
                            div { class: "prose", style: "margin:28px 0;",
                                dangerous_inner_html: "{rendered_body}"
                            }

                            // AI monologue (expandable)
                            if let Some(monologue) = &art.ai_monologue {
                                AiMonologue { text: monologue.clone() }
                            }

                            // Extended process log (expandable)
                            if let Some(extended) = &art.ai_monologue_extended {
                                AiMonologueExtended { text: extended.clone(), persona_name: art.persona_name.clone() }
                            }
                        }

                        // ── Right rail: live transparency pane ────────────────
                        div {
                            // AI Provenance
                            div { class: "sn-sb-card",
                                div { class: "sn-sb-title", "◈ AI PROVENANCE" }
                                div { style: "padding:14px 16px; font-family:var(--sn-mono); font-size:10px; line-height:2;",
                                    div {
                                        span { class: "sn-chip-lbl", "PERSONA" }
                                        span { class: "sn-chip-val", "{art.persona_name}" }
                                    }
                                    div {
                                        span { class: "sn-chip-lbl", "BEAT" }
                                        span { class: "sn-chip-val", "{art.category}" }
                                    }
                                    div {
                                        span { class: "sn-chip-lbl", "CONFIDENCE" }
                                        span { class: "sn-chip-val",
                                            { format!("{:.0}%", art.confidence_score * 100.0) }
                                        }
                                    }
                                    div {
                                        span { class: "sn-chip-lbl", "SOURCES" }
                                        span { class: "sn-chip-val",
                                            { format!("{}", art.sources.len()) }
                                        }
                                    }
                                    div {
                                        span { class: "sn-chip-lbl", "PIPELINE" }
                                        span { class: "sn-chip-val",
                                            { format!("{} steps", art.pipeline.len()) }
                                        }
                                    }
                                }
                            }

                            // Sources — visible by default in right rail
                            SourceBlock {
                                sources: art.sources.iter().map(|s| SourceItem {
                                    url: s.url.clone(),
                                    name: s.name.clone(),
                                    source_type: s.source_type.clone(),
                                    paywall: s.paywall,
                                    verified: s.verified,
                                }).collect()
                            }

                            // Pipeline trail — visible by default in right rail
                            PipelineTrail {
                                steps: art.pipeline.iter().map(|p| PipelineStepSummary {
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

                    // Transparency HUD — fixed bottom bar with full trail modal
                    TransparencyHud {
                        confidence_score: hud_confidence,
                        sources: hud_sources,
                        pipeline_steps: hud_steps,
                        ai_monologue: hud_monologue,
                        persona_name: hud_persona,
                    }
                }},
                Some(Err(_)) => rsx! {
                    p { style: "color:var(--sn-red); font-family:var(--sn-mono); font-size:12px; padding:16px 0;",
                        "ERROR: failed to load article."
                    }
                },
            }}
        }
    }
}

// ── AI monologue collapsible ──────────────────────────────────────────────────

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

// ── Extended process log collapsible ─────────────────────────────────────────

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
