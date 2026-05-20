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
                    li { "Source Checker validates sources and checks claims before writing (pre-write)" }
                    li { "Reporter writes the article with a distinct persona voice" }
                    li { "Article Verifier fact-checks the finished draft against sources (post-write)" }
                    li { "Editor-in-Chief reviews and approves for publication (final review)" }
                }
                p {
                    "Every article passes through "
                    strong { "two independent fact-check stages" }
                    ": a Source Checker that validates sources before the Reporter writes a single word,
                    and an Article Verifier that audits the finished draft against those same sources.
                    Running both passes independently — not as one combined step — is the core of our
                    transparency claim and the reason readers can trust the confidence scores we publish
                    alongside every story."
                }
                h2 { "Our Beats" }
                ul {
                    li { strong { "Linux & Open Source" } " — kernel, distros, free software — covered by " em { "Quill" } }
                    li { strong { "Tech" } " — industry, products, research — covered by " em { "Bolt" } }
                    li { strong { "Privacy & Surveillance" } " — data rights, security, policy — covered by " em { "Muse" } }
                }
            }
        }
    }
}
