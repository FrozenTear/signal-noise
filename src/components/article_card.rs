use dioxus::prelude::*;
use crate::util::simple_md_to_html;

#[derive(Props, Clone, PartialEq)]
pub struct ArticleCardProps {
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub category: String,
    #[props(default)]
    pub region: String,
    pub persona_name: String,
    pub confidence_score: f64,
    pub published_at: String,
    #[props(default)]
    pub ai_monologue: Option<String>,
    #[props(default)]
    pub ai_monologue_extended: Option<String>,
    #[props(default)]
    pub is_featured: bool,
    #[props(default)]
    pub source_count: Option<u32>,
    #[props(default)]
    pub pipeline_step_count: Option<u32>,
}

#[component]
pub fn ArticleCard(props: ArticleCardProps) -> Element {
    let is_featured = props.is_featured;
    let mut show_extended = use_signal(move || is_featured);

    let acc = props.confidence_score;
    let acc_pct = (acc * 100.0) as u32;
    let acc_cls = conf_class(acc);

    let beat_cls = match props.category.as_str() {
        "linux"   => "linux",
        "privacy" => "privacy",
        _         => "tech",
    };

    let monologue = match props.ai_monologue.as_deref() {
        Some(full) => full.trim().to_string(),
        None => monologue_hook(&props.persona_name, &props.title),
    };

    let pipeline_label = props.pipeline_step_count.map(|n| format!("{n}-step pipeline"));
    let src_label = props.source_count.map(|n| {
        if n == 1 { "1 source".to_string() } else { format!("{n} sources") }
    });

    let article_class = if props.is_featured {
        format!("sn-article beat-{beat_cls} sn-featured")
    } else {
        format!("sn-article beat-{beat_cls}")
    };

    let headline_style = if props.is_featured { "font-size:36px;" } else { "" };

    rsx! {
        article { class: "{article_class}",

            div { class: "sn-article-inner",
                // Meta row: beat tag + region tag + timestamp + confidence + byline
                div { style: "display:flex;align-items:center;gap:10px;margin-bottom:14px;flex-wrap:wrap;",
                    span { class: "sn-beat-tag {beat_cls}", "{props.category}" }
                    if !props.region.is_empty() && props.region != "global" {
                        span { class: "sn-region-tag sn-region-tag--{props.region}", "{props.region}" }
                    }
                    span { class: "sn-ts", "{props.published_at}" }
                    span { style: "display:inline-flex;align-items:center;gap:5px;font-family:var(--sn-mono);font-size:9px;",
                        span { style: "color:var(--sn-text-dimmer);", "confidence" }
                        span { class: "sn-conf-val {acc_cls}", "{acc_pct}%" }
                    }
                    span { style: "margin-left:auto;font-family:var(--sn-mono);font-size:9px;color:var(--sn-text-dimmer);",
                        "by {props.persona_name}"
                    }
                }

                // Headline
                h2 { class: "sn-headline", style: "{headline_style}",
                    a { href: "/articles/{props.slug}", "{props.title}" }
                }

                // Pipeline fingerprint strip
                div { class: "sn-fingerprint",
                    span { style: "color:var(--sn-violet);", "◈" }
                    if let Some(label) = &pipeline_label {
                        span { class: "sn-gen-pill temp", "{label}" }
                    }
                    if let Some(label) = &src_label {
                        span { class: "sn-gen-pill tokens", "{label}" }
                    }
                    span { class: "sn-gen-pill temp",
                        span { class: "sn-conf-val {acc_cls}", "{acc_pct}%" }
                        " confidence"
                    }
                }

                // Summary
                p { class: "sn-summary", "{props.summary}" }

                // AI internal monologue
                div { class: "sn-monologue",
                    div { class: "sn-monologue-label", "AI reasoning · {props.persona_name}" }
                    "{monologue}"
                }

                // Extended monologue (expandable; open by default for featured)
                if let Some(extended) = &props.ai_monologue_extended {
                    {
                        let rendered_extended = simple_md_to_html(extended);
                        rsx! {
                            button {
                                class: "sn-toggle-btn",
                                onclick: move |_| show_extended.set(!show_extended()),
                                span { style: "color:var(--sn-violet);", "◈" }
                                if show_extended() { " hide full reasoning trace" } else { " show full reasoning trace" }
                            }
                            if show_extended() {
                                div { class: "sn-monologue sn-monologue-extended", style: "margin-top:6px;",
                                    div { class: "sn-monologue-label", "full AI reasoning trace · {props.persona_name}" }
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
