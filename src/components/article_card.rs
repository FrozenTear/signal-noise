use dioxus::prelude::*;
use crate::util::simple_md_to_html;

#[derive(Props, Clone, PartialEq)]
pub struct ArticleCardProps {
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub category: String,
    pub persona_name: String,
    pub confidence_score: f64,
    pub published_at: String,
    #[props(default)]
    pub ai_monologue: Option<String>,
    #[props(default)]
    pub ai_monologue_extended: Option<String>,
    #[props(default)]
    pub source_substitution: bool,
}

#[component]
pub fn ArticleCard(props: ArticleCardProps) -> Element {
    let mut show_extended = use_signal(|| false);

    let acc = props.confidence_score;
    let acc_pct = (acc * 100.0) as u32;
    let acc_cls = conf_class(acc);

    let beat_cls = match props.category.as_str() {
        "linux"   => "linux",
        "privacy" => "privacy",
        _         => "tech",
    };

    // Use real monologue from DB when available; show full text now that monologues are concise
    let monologue = match props.ai_monologue.as_deref() {
        Some(full) => full.trim().to_string(),
        None => monologue_hook(&props.persona_name, &props.title),
    };

    rsx! {
        article { class: "sn-article beat-{beat_cls}",

            div { class: "sn-article-inner",
                // Meta row: beat tag + timestamp + confidence + byline
                div { style: "display:flex;align-items:center;gap:10px;margin-bottom:14px;flex-wrap:wrap;",
                    span { class: "sn-beat-tag {beat_cls}", "{props.category}" }
                    span { class: "sn-ts", "{props.published_at}" }
                    // Inline confidence score
                    span { style: "display:inline-flex;align-items:center;gap:5px;font-family:var(--sn-mono);font-size:9px;",
                        span { style: "color:var(--sn-text-dimmer);", "confidence" }
                        span { class: "sn-conf-val {acc_cls}", "{acc_pct}%" }
                    }
                    if props.source_substitution {
                        span { style: "display:inline-flex;align-items:center;gap:3px;font-family:var(--sn-mono);font-size:8px;color:var(--sn-violet);border:1px solid var(--sn-violet);border-radius:3px;padding:1px 5px;",
                            "◈ SRC SUBST"
                        }
                    }
                    span { style: "margin-left:auto;font-family:var(--sn-mono);font-size:9px;color:var(--sn-text-dimmer);",
                        "by {props.persona_name}"
                    }
                }

                // Headline
                h2 { class: "sn-headline",
                    a { href: "/article/{props.slug}", "{props.title}" }
                }

                // Summary
                p { class: "sn-summary", "{props.summary}" }

                // AI internal monologue (short)
                div { class: "sn-monologue",
                    div { class: "sn-monologue-label", "internal monologue · {props.persona_name}" }
                    "{monologue}"
                }

                // Extended monologue (expandable)
                if let Some(extended) = &props.ai_monologue_extended {
                    {
                        let rendered_extended = simple_md_to_html(extended);
                        rsx! {
                            button {
                                class: "sn-toggle-btn",
                                onclick: move |_| show_extended.set(!show_extended()),
                                span { style: "color:var(--sn-violet);", "◈" }
                                if show_extended() { " hide full process log" } else { " show full process log" }
                            }
                            if show_extended() {
                                div { class: "sn-monologue sn-monologue-extended", style: "margin-top:6px;",
                                    div { class: "sn-monologue-label", "extended internal monologue · {props.persona_name}" }
                                    div { dangerous_inner_html: "{rendered_extended}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn conf_class(score: f64) -> &'static str {
    if score >= 0.80 { "g" } else if score >= 0.60 { "a" } else { "r" }
}

/// Short, catchy persona hooks for the card view. One sentence, max personality.
fn monologue_hook(persona: &str, _title: &str) -> String {
    match persona {
        "Priya Chandran" =>
            "Every press release says \u{2018}revolutionary.\u{2019} I count how many survive the week."
                .to_string(),
        "Milo Varga" =>
            "Another kernel patch, another mass extinction of workarounds I was personally fond of."
                .to_string(),
        "Sable Ren" =>
            "They said the data was anonymized. I found the receipt."
                .to_string(),
        // Legacy persona fallbacks
        "Linus Watcher" =>
            "I remain cautiously descriptive. The kernel does not share this restraint."
                .to_string(),
        "Panoptikon" =>
            "Primary sources and official statements do not always describe the same reality."
                .to_string(),
        "Circuit Breaker" =>
            "The pricing says \u{2018}starting at.\u{2019} Marketing language for \u{2018}the version you want costs more.\u{2019}"
                .to_string(),
        _ =>
            "My confidence score reflects the sources I found, not the sources that exist. There is a difference."
                .to_string(),
    }
}
