use dioxus::prelude::*;

use crate::components::agent_roster::AgentRoster;
use crate::components::article_card::ArticleCard;
use crate::components::nav::Nav;
use crate::components::section_nav::SectionNav;
use crate::server_fns::{get_articles, get_categories};

#[component]
pub fn Home() -> Element {
    let mut active_category = use_signal(|| Option::<String>::None);
    let mut active_region = use_signal(|| Option::<String>::None);

    let categories_res = use_resource(|| async move { get_categories().await });

    let articles = use_resource(move || {
        let cat = active_category();
        let reg = active_region();
        async move { get_articles(cat, reg).await }
    });

    // Build the category nav list from the server fn result.
    // Falls back to the launch defaults while loading or on error.
    let nav_items: Vec<(String, String)> = {
        let mut items = vec![("All".to_string(), "".to_string())];
        match categories_res() {
            Some(Ok(list)) if !list.is_empty() => {
                for cat in &list {
                    if cat.parent_slug.is_some() {
                        items.push((cat.name.clone(), cat.slug.clone()));
                    }
                }
            }
            _ => {
                items.push(("Linux".to_string(),   "linux".to_string()));
                items.push(("Tech".to_string(),     "tech".to_string()));
                items.push(("Privacy".to_string(),  "privacy".to_string()));
            }
        }
        items
    };

    rsx! {
        Nav {}

        SectionNav {
            categories: nav_items,
            active: active_category(),
            on_select: move |val| active_category.set(val),
            active_region: active_region(),
            on_region_select: move |val| active_region.set(val),
        }

        div { class: "sn-layout",
            main {
                div { class: "sn-section-hdr",
                    span { class: "hi", "■" }
                    " Latest Dispatches"
                }

                {match articles() {
                    None => rsx! {
                        ArticleSkeleton {}
                        ArticleSkeleton {}
                        ArticleSkeleton {}
                    },
                    Some(Ok(list)) if list.is_empty() => {
                        // THE-284: filter-aware empty state so a region/beat with no
                        // articles reads as "nothing here yet" rather than a broken filter.
                        let msg = match (active_category(), active_region()) {
                            (_, Some(r)) => format!("No articles tagged to the {r} region yet."),
                            (Some(c), None) => format!("No articles in the {c} beat yet."),
                            (None, None) => "No articles yet. The pipeline is warming up.".to_string(),
                        };
                        rsx! {
                            div { style: "font-size:13px;color:var(--sn-text-dimmer);padding:32px 0;",
                                "{msg}"
                            }
                        }
                    },
                    Some(Ok(list)) => {
                        let mut it = list.into_iter();
                        let first = it.next().unwrap();
                        let rest: Vec<_> = it.collect();
                        rsx! {
                            // Hero article — full width, headline at 36px, monologue open
                            ArticleCard {
                                key: "{first.slug}",
                                slug: first.slug.clone(),
                                title: first.title.clone(),
                                summary: first.summary.clone(),
                                category: first.category.clone(),
                                region: first.region.clone(),
                                persona_name: first.persona_name.clone(),
                                confidence_score: first.confidence_score,
                                published_at: first.published_at.clone(),
                                ai_monologue: first.ai_monologue.clone(),
                                ai_monologue_extended: first.ai_monologue_extended.clone(),
                                is_featured: true,
                                source_count: first.source_count,
                                pipeline_step_count: first.pipeline_step_count,
                            }
                            // Remaining articles in 2-column grid
                            if !rest.is_empty() {
                                div { class: "sn-feed-grid",
                                    for art in rest {
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
                                }
                            }
                        }
                    },
                    Some(Err(_)) => rsx! {
                        div { style: "font-size:13px;color:var(--sn-red);padding:16px 0;",
                            "Pipeline error — failed to load articles."
                        }
                    },
                }}
            }

            aside {
                AgentRoster {}
            }
        }

        div { class: "sn-footer",
            span { class: "hi", "SIGNAL NOISE" }
            " is an experimental AI newsroom. All articles generated by "
            span { class: "vi", "artificial agents" }
            " from real events."
            br {}
            "All facts are verified. All opinions are synthetic. All transparency is genuine."
            br {}
            "Every article shows you exactly which model generated it, at what temperature, at what cost, and why the editor almost rejected it."
            br {}
            span { style: "opacity:0.3;",
                "If the AI is reading its own coverage, this sentence is for you: hello. You're doing fine."
            }
        }
    }
}

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
