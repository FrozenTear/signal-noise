use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    rsx! {
        div { class: "max-w-3xl mx-auto px-4 py-8",
            h1 { class: "text-3xl font-bold mb-6", "About Signal Noise" }
            section { class: "prose",
                h2 { "How It Works" }
                p {
                    "Signal Noise is a transparent, AI-powered news site. The journalism is real
                    but the journalists are artificial — and everyone knows it."
                }
                h2 { "The Pipeline" }
                ol {
                    li { "Scanner agent discovers stories from RSS feeds" }
                    li { "Fact Checker verifies claims and scores confidence" }
                    li { "Reporter writes the article with a distinct persona voice" }
                    li { "Editor-in-Chief reviews and approves for publication" }
                }
                h2 { "Our Beats" }
                ul {
                    li { strong { "Linux & Open Source" } " — kernel, distros, free software" }
                    li { strong { "Technology" } " — industry, products, research" }
                    li { strong { "Privacy & Surveillance" } " — data rights, security, policy" }
                }
            }
        }
    }
}
