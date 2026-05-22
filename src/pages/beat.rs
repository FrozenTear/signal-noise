use dioxus::prelude::*;

use crate::components::article_card::ArticleCard;
use crate::components::nav::Nav;
use crate::server_fns::{get_articles, get_categories};

#[component]
fn ArticleSkeleton() -> Element {
    rsx! {
        div { class: "sn-skeleton",
            div { class: "sn-skeleton-bar", style: "width:30%;margin-bottom:14px" }
            div { class: "sn-skeleton-bar", style: "width:85%;height:14px;margin-bottom:8px" }
            div { class: "sn-skeleton-bar", style: "width:70%;height:14px;margin-bottom:20px" }
            div { class: "sn-skeleton-bar", style: "width:100%" }
            div { class: "sn-skeleton-bar", style: "width:90%" }
            div { class: "sn-skeleton-bar", style: "width:60%" }
        }
    }
}

// ── Backwards-compatible aliases ──────────────────────────────────────────────

#[component]
pub fn BeatLinux() -> Element {
    rsx! { Beat { category: "linux".to_string(), title: "Linux & Open Source".to_string() } }
}

#[component]
pub fn BeatTech() -> Element {
    rsx! { Beat { category: "tech".to_string(), title: "Technology".to_string() } }
}

#[component]
pub fn BeatPrivacy() -> Element {
    rsx! { Beat { category: "privacy".to_string(), title: "Privacy & Surveillance".to_string() } }
}

// ── Generic beat page (data-driven) ──────────────────────────────────────────

/// Route handler for /beat/:slug — resolves the display title from get_categories().
#[component]
pub fn BeatPage(slug: String) -> Element {
    let slug_for_res = slug.clone();
    let categories_res = use_resource(move || {
        let s = slug_for_res.clone();
        async move {
            let cats = get_categories().await.unwrap_or_default();
            cats.into_iter().find(|c| c.slug == s)
        }
    });

    let title = match categories_res() {
        Some(Some(cat)) => cat.name.clone(),
        _ => {
            let mut t = slug.clone();
            if let Some(first) = t.get_mut(0..1) {
                first.make_ascii_uppercase();
            }
            t
        }
    };

    rsx! { Beat { category: slug, title } }
}

// ── Shared beat page body ─────────────────────────────────────────────────────

#[component]
fn Beat(category: String, title: String) -> Element {
    let cat_clone = category.clone();
    let articles = use_resource(move || {
        let cat = cat_clone.clone();
        async move { get_articles(Some(cat), None).await }
    });

    let beat_cls = match category.as_str() {
        "linux" => "linux",
        "privacy" => "privacy",
        _ => "tech",
    };

    rsx! {
        Nav {}
        div { class: "sn-layout", style: "grid-template-columns:1fr;",
            main {
                div { style: "display:flex;align-items:center;gap:14px;margin-bottom:24px;",
                    span { class: "sn-beat-tag {beat_cls}", style: "font-size:10px;padding:4px 12px;", "{category}" }
                    h1 { class: "sn-headline", style: "margin-bottom:0;", "{title}" }
                }
                p { style: "font-size:15px;color:var(--sn-text-dim);margin-bottom:28px;line-height:1.6;",
                    "AI-generated coverage of {category} news."
                }

                div { class: "sn-section-hdr",
                    span { class: "hi", "■" }
                    " Latest Dispatches"
                }

                {match articles() {
                    None => rsx! {
                        ArticleSkeleton {}
                        ArticleSkeleton {}
                    },
                    Some(Ok(list)) if list.is_empty() => rsx! {
                        div { style: "font-family:var(--sn-mono);font-size:12px;color:var(--sn-text-dimmer);padding:32px 0;",
                            "No {category} articles yet. The pipeline is warming up."
                        }
                    },
                    Some(Ok(list)) => rsx! {
                        for art in list {
                            ArticleCard {
                                key: "{art.slug}",
                                slug: art.slug.clone(),
                                title: art.title.clone(),
                                summary: art.summary.clone(),
                                category: art.category.clone(),
                                region: art.region.clone(),
                                persona_name: art.persona_name.clone(),
                                confidence_score: art.confidence_score,
                                published_at: art.published_at.clone(),
                                ai_monologue: art.ai_monologue.clone(),
                                ai_monologue_extended: art.ai_monologue_extended.clone(),
                                source_count: art.source_count,
                                pipeline_step_count: art.pipeline_step_count,
                            }
                        }
                    },
                    Some(Err(_)) => rsx! {
                        div { style: "font-family:var(--sn-mono);font-size:12px;color:var(--sn-red);padding:16px 0;",
                            "Pipeline error — failed to load articles."
                        }
                    },
                }}
            }
        }

        div { class: "sn-footer",
            span { class: "hi", "SIGNAL NOISE" }
            " · AI-powered newsroom · All facts verified · All transparency genuine"
        }
    }
}
