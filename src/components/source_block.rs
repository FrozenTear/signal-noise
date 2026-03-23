use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct SourceItem {
    pub url: String,
    pub name: String,
    pub source_type: String,
    pub paywall: bool,
    pub verified: bool,
}

#[derive(Props, Clone, PartialEq)]
pub struct SourceBlockProps {
    pub sources: Vec<SourceItem>,
}

#[component]
pub fn SourceBlock(props: SourceBlockProps) -> Element {
    rsx! {
        div { class: "border-t border-gray-200 mt-8 pt-4",
            h3 { class: "text-sm font-semibold text-gray-700 mb-3", "Sources" }
            if props.sources.is_empty() {
                p { class: "text-sm text-gray-400", "No sources recorded." }
            } else {
                ul { class: "space-y-2",
                    for source in &props.sources {
                        li { class: "flex items-center gap-2 text-sm",
                            a { class: "text-blue-600 hover:underline", href: "{source.url}", "{source.name}" }
                            span { class: "text-xs text-gray-400", "[{source.source_type}]" }
                            if source.paywall {
                                span { class: "text-xs text-orange-500", "paywall" }
                            }
                            if source.verified {
                                span { class: "text-xs text-green-600", "verified" }
                            }
                        }
                    }
                }
            }
        }
    }
}
