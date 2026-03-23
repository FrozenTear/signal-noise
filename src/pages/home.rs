use dioxus::prelude::*;

use crate::components::agent_roster::AgentRoster;
use crate::components::article_card::ArticleCard;
use crate::components::nav::Nav;
use crate::server_fns::get_articles;

#[component]
pub fn Home() -> Element {
    let mut active_category = use_signal(|| Option::<String>::None);

    let articles = use_resource(move || {
        let cat = active_category();
        async move { get_articles(cat).await }
    });

    rsx! {
        Nav {}
        div { class: "max-w-5xl mx-auto px-4 py-8",
            // Beat filter tabs
            div { class: "flex gap-2 mb-8",
                CategoryTab {
                    label: "All",
                    active: active_category().is_none(),
                    onclick: move |_| active_category.set(None),
                }
                CategoryTab {
                    label: "Linux",
                    active: active_category() == Some("linux".to_string()),
                    onclick: move |_| active_category.set(Some("linux".to_string())),
                }
                CategoryTab {
                    label: "Tech",
                    active: active_category() == Some("tech".to_string()),
                    onclick: move |_| active_category.set(Some("tech".to_string())),
                }
                CategoryTab {
                    label: "Privacy",
                    active: active_category() == Some("privacy".to_string()),
                    onclick: move |_| active_category.set(Some("privacy".to_string())),
                }
            }

            div { class: "flex gap-8",
                main { class: "flex-1 min-w-0",
                    {match articles() {
                        None => rsx! {
                            div { class: "space-y-4",
                                ArticleSkeleton {}
                                ArticleSkeleton {}
                                ArticleSkeleton {}
                            }
                        },
                        Some(Ok(list)) if list.is_empty() => rsx! {
                            p { class: "text-gray-400 text-sm py-8",
                                "No articles yet. The pipeline is warming up."
                            }
                        },
                        Some(Ok(list)) => rsx! {
                            div { class: "space-y-6",
                                for art in list {
                                    ArticleCard {
                                        key: "{art.slug}",
                                        slug: art.slug.clone(),
                                        title: art.title.clone(),
                                        summary: art.summary.clone(),
                                        category: art.category.clone(),
                                        persona_name: art.persona_name.clone(),
                                        confidence_score: art.confidence_score,
                                        published_at: art.published_at.clone(),
                                    }
                                }
                            }
                        },
                        Some(Err(_)) => rsx! {
                            p { class: "text-red-500 text-sm py-4",
                                "Failed to load articles."
                            }
                        },
                    }}
                }
                aside { class: "w-64 shrink-0",
                    AgentRoster {}
                }
            }
        }
    }
}

// ── Sub-components ────────────────────────────────────────────────────────────

#[derive(Props, Clone, PartialEq)]
struct CategoryTabProps {
    label: &'static str,
    active: bool,
    onclick: EventHandler<MouseEvent>,
}

#[component]
fn CategoryTab(props: CategoryTabProps) -> Element {
    let class = if props.active {
        "px-3 py-1 text-sm font-medium bg-gray-900 text-white rounded"
    } else {
        "px-3 py-1 text-sm font-medium text-gray-600 hover:text-gray-900 rounded border border-gray-200"
    };
    rsx! {
        button {
            class: "{class}",
            onclick: move |e| props.onclick.call(e),
            "{props.label}"
        }
    }
}

#[component]
fn ArticleSkeleton() -> Element {
    rsx! {
        div { class: "border border-gray-100 rounded-lg p-4 animate-pulse",
            div { class: "flex gap-2 mb-3",
                div { class: "h-3 bg-gray-100 rounded w-16" }
                div { class: "h-3 bg-gray-100 rounded w-20" }
            }
            div { class: "h-5 bg-gray-100 rounded w-3/4 mb-2" }
            div { class: "h-3 bg-gray-100 rounded w-full mb-1" }
            div { class: "h-3 bg-gray-100 rounded w-2/3 mb-4" }
            div { class: "flex gap-4",
                div { class: "h-3 bg-gray-100 rounded w-24" }
                div { class: "h-3 bg-gray-100 rounded w-20" }
            }
        }
    }
}
