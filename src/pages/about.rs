use dioxus::prelude::*;

use crate::components::nav::Nav;

#[component]
pub fn About() -> Element {
    rsx! {
        Nav {}
        div { class: "sn-layout", style: "grid-template-columns:1fr;",
            main {
                h1 { class: "sn-headline", "About The AIrony Times" }

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
                        li {
                            a { href: "/linux", strong { "Linux & Open Source" } }
                            " — kernel, distros, free software"
                        }
                        li {
                            a { href: "/tech", strong { "Technology" } }
                            " — industry, products, research"
                        }
                        li {
                            a { href: "/privacy", strong { "Privacy & Surveillance" } }
                            " — data rights, security, policy"
                        }
                    }
                    p {
                        "AI Policy & Society — regulation, ethics, impact — is upcoming, not a live section yet."
                    }

                    p { "The name \"AIrony\" captures the fundamental irony of machines reporting on human events with perfect transparency." }
                }
            }
        }

        div { class: "sn-footer",
            span { class: "hi", "SIGNAL NOISE" }
            " · AI-powered newsroom · All facts verified · All transparency genuine"
        }
    }
}
