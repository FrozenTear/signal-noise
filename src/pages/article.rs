use dioxus::prelude::*;

use crate::components::confidence_meter::ConfidenceMeter;
use crate::components::nav::Nav;
use crate::components::pipeline_trail::{PipelineStepSummary, PipelineTrail};
use crate::components::source_block::{SourceBlock, SourceItem};
use crate::server_fns::get_article_by_slug;

#[component]
pub fn Article(slug: String) -> Element {
    let slug_copy = slug.clone();
    let article = use_resource(move || {
        let s = slug_copy.clone();
        async move { get_article_by_slug(s).await }
    });

    rsx! {
        Nav {}
        div { class: "max-w-3xl mx-auto px-4 py-8",
            {match article() {
                None => rsx! {
                    div { class: "animate-pulse space-y-4",
                        div { class: "h-4 bg-gray-100 rounded w-24 mb-6" }
                        div { class: "h-8 bg-gray-100 rounded w-5/6 mb-2" }
                        div { class: "h-8 bg-gray-100 rounded w-4/6 mb-6" }
                        div { class: "h-3 bg-gray-100 rounded w-full mb-1" }
                        div { class: "h-3 bg-gray-100 rounded w-full mb-1" }
                        div { class: "h-3 bg-gray-100 rounded w-3/4" }
                    }
                },
                Some(Ok(None)) => rsx! {
                    div { class: "py-16 text-center",
                        h2 { class: "text-xl font-semibold text-gray-700 mb-2", "Article not found" }
                        p { class: "text-gray-400 text-sm", "This article may have been removed or the URL is incorrect." }
                        a { class: "text-blue-600 text-sm hover:underline mt-4 inline-block",
                            href: "/",
                            "Back to home"
                        }
                    }
                },
                Some(Ok(Some(art))) => rsx! {
                    // Byline + category
                    div { class: "flex items-center gap-2 mb-4 text-sm text-gray-500",
                        span { class: "uppercase tracking-wide text-xs font-medium text-gray-400",
                            "{art.category}"
                        }
                        span { "·" }
                        span { "{art.published_at}" }
                        span { "·" }
                        span { "by {art.persona_name}" }
                    }

                    // Title
                    h1 { class: "text-3xl font-bold leading-tight mb-4", "{art.title}" }

                    // Confidence meter
                    ConfidenceMeter { score: art.confidence_score }

                    // AI content disclaimer
                    div { class: "bg-yellow-50 border border-yellow-200 rounded p-3 my-4 text-xs text-yellow-800",
                        "This article was written entirely by AI agents. All claims have been \
                         fact-checked by a separate AI process. See sources and editorial trail below."
                    }

                    // Article body
                    div { class: "prose prose-gray max-w-none my-8 text-gray-800 leading-relaxed whitespace-pre-wrap",
                        "{art.body}"
                    }

                    // AI monologue (expandable)
                    if let Some(monologue) = &art.ai_monologue {
                        AiMonologue { text: monologue.clone() }
                    }

                    // Sources
                    SourceBlock {
                        sources: art.sources.iter().map(|s| SourceItem {
                            url: s.url.clone(),
                            name: s.name.clone(),
                            source_type: s.source_type.clone(),
                            paywall: s.paywall,
                            verified: s.verified,
                        }).collect()
                    }

                    // Pipeline trail
                    PipelineTrail {
                        steps: art.pipeline.iter().map(|p| PipelineStepSummary {
                            agent_name: p.agent_name.clone(),
                            step_type: p.step_type.clone(),
                            output_summary: p.output_summary.clone(),
                            confidence_delta: p.confidence_delta,
                            completed_at: p.completed_at.clone(),
                        }).collect()
                    }
                },
                Some(Err(_)) => rsx! {
                    p { class: "text-red-500 text-sm py-4", "Failed to load article." }
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

    rsx! {
        div { class: "border border-gray-200 rounded-lg my-6",
            button {
                class: "w-full flex items-center justify-between px-4 py-3 text-sm font-medium text-gray-700 hover:bg-gray-50",
                onclick: move |_| open.toggle(),
                span { "AI internal monologue" }
                span { class: "text-gray-400 text-xs",
                    if open() { "hide" } else { "show" }
                }
            }
            if open() {
                div { class: "px-4 pb-4 text-sm text-gray-600 italic border-t border-gray-100 pt-3",
                    "{props.text}"
                }
            }
        }
    }
}
