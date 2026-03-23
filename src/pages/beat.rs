use dioxus::prelude::*;

use crate::components::article_card::ArticleCard;
use crate::components::nav::Nav;
use crate::server_fns::get_articles;

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

#[component]
fn Beat(category: String, title: String) -> Element {
    let cat_clone = category.clone();
    let articles = use_resource(move || {
        let cat = cat_clone.clone();
        async move { get_articles(Some(cat)).await }
    });

    rsx! {
        Nav {}
        div { class: "max-w-3xl mx-auto px-4 py-8",
            h1 { class: "text-2xl font-bold mb-2", "{title}" }
            p { class: "text-sm text-gray-500 mb-8",
                "AI-generated coverage of {category} news."
            }

            {match articles() {
                None => rsx! {
                    div { class: "space-y-4",
                        div { class: "border border-gray-100 rounded-lg p-4 animate-pulse h-24" }
                        div { class: "border border-gray-100 rounded-lg p-4 animate-pulse h-24" }
                    }
                },
                Some(Ok(list)) if list.is_empty() => rsx! {
                    p { class: "text-gray-400 text-sm py-8",
                        "No {category} articles yet."
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
                    p { class: "text-red-500 text-sm", "Failed to load articles." }
                },
            }}
        }
    }
}
