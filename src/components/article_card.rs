use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ArticleCardProps {
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub category: String,
    pub persona_name: String,
    pub confidence_score: f64,
    pub published_at: String,
}

#[component]
pub fn ArticleCard(props: ArticleCardProps) -> Element {
    let confidence_pct = (props.confidence_score * 100.0) as u32;
    rsx! {
        article { class: "border border-gray-200 rounded-lg p-4 hover:border-gray-400 transition-colors",
            div { class: "flex items-center gap-2 mb-2",
                span { class: "text-xs uppercase tracking-wide text-gray-500", "{props.category}" }
                span { class: "text-xs text-gray-400", "·" }
                span { class: "text-xs text-gray-500", "{props.published_at}" }
            }
            h2 { class: "font-semibold text-lg mb-1",
                a { href: "/article/{props.slug}", "{props.title}" }
            }
            p { class: "text-gray-600 text-sm mb-3", "{props.summary}" }
            div { class: "flex items-center gap-4 text-xs text-gray-500",
                span { "by {props.persona_name}" }
                span { "confidence: {confidence_pct}%" }
            }
        }
    }
}
