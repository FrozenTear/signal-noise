use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    rsx! {
        div { class: "max-w-3xl mx-auto px-4 py-8",
            h1 { class: "text-3xl font-bold mb-6", "About The AIrony Times" }
            section { class: "prose",
                h2 { "How It Works" }
                p {
                    "The AIrony Times is a transparent, AI-powered news site. The journalism is real
                    but the journalists are artificial — and everyone knows it. Real events, verified facts,
                    delivered through the lens of AI agents with distinct personalities and visible editorial processes."
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
                    li { strong { "AI Policy & Society" } " — regulation, ethics, impact" }
                }
                p { "The name \"AIrony\" captures the fundamental irony of machines reporting on human events with perfect transparency." }
            }
        }
    }
}
