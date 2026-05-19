use dioxus::prelude::*;
use crate::util::simple_md_to_html;

#[derive(Clone, PartialEq)]
pub struct HudSource {
    pub name: String,
    pub verified: bool,
}

#[derive(Clone, PartialEq)]
pub struct HudPipelineStep {
    pub step_type: String,
    pub confidence_delta: f64,
}

#[derive(Props, Clone, PartialEq)]
pub struct TransparencyHudProps {
    pub confidence_score: f64,
    pub sources: Vec<HudSource>,
    pub pipeline_steps: Vec<HudPipelineStep>,
    #[props(default)]
    pub ai_monologue: Option<String>,
    #[props(default)]
    pub persona_name: Option<String>,
}

#[component]
pub fn TransparencyHud(props: TransparencyHudProps) -> Element {
    let mut modal_open = use_signal(|| false);

    let confidence_pct = (props.confidence_score * 100.0) as u32;
    let conf_cls = conf_color_class(props.confidence_score);
    let verified_count = props.sources.iter().filter(|s| s.verified).count();
    let total_sources = props.sources.len();
    let unverified_count = total_sources.saturating_sub(verified_count);
    let step_count = props.pipeline_steps.len();

    rsx! {
        // ── Fixed bottom HUD bar ──────────────────────────────────────────────
        div { class: "sn-transparency-hud",

            div { class: "sn-hud-section",
                span { class: "sn-hud-label", "CONFIDENCE" }
                span { class: "sn-conf-val {conf_cls}", style: "font-size:15px;font-weight:600;",
                    "{confidence_pct}%"
                }
            }

            div { class: "sn-hud-section",
                span { class: "sn-hud-label", "SOURCES" }
                div { class: "sn-hud-pills",
                    if verified_count > 0 {
                        span { class: "sn-hud-pill verified", "{verified_count} verified" }
                    }
                    if unverified_count > 0 {
                        span { class: "sn-hud-pill unverified", "{unverified_count} other" }
                    }
                    if total_sources == 0 {
                        span { class: "sn-hud-pill unverified", "no sources" }
                    }
                }
            }

            div { class: "sn-hud-section sn-hud-steps-section",
                span { class: "sn-hud-label", "PIPELINE" }
                div { class: "sn-hud-steps",
                    for (i, step) in props.pipeline_steps.iter().enumerate() {
                        {
                            let abbrev = step_abbrev(&step.step_type);
                            let is_last = i == step_count - 1;
                            rsx! {
                                span { class: "sn-hud-step", title: "{step.step_type}", "{abbrev}" }
                                if !is_last {
                                    span { class: "sn-hud-step-sep", "›" }
                                }
                            }
                        }
                    }
                    if step_count == 0 {
                        span { class: "sn-hud-pill unverified", "no trail" }
                    }
                }
            }

            div { class: "sn-hud-spacer" }

            button {
                class: "sn-hud-expand",
                onclick: move |_| modal_open.set(true),
                span { class: "sn-hud-expand-full", "VIEW TRANSPARENCY TRAIL →" }
                span { class: "sn-hud-expand-short", "TRAIL →" }
            }
        }

        // ── Modal overlay ─────────────────────────────────────────────────────
        if modal_open() {
            div {
                class: "sn-hud-modal-overlay",
                onclick: move |_| modal_open.set(false),

                div {
                    class: "sn-hud-modal",
                    onclick: move |e| e.stop_propagation(),

                    // Header
                    div { class: "sn-hud-modal-hdr",
                        span { class: "sn-hud-modal-title",
                            span { style: "color:var(--sn-accent);", "◈" }
                            " TRANSPARENCY TRAIL"
                        }
                        if let Some(persona) = &props.persona_name {
                            span { style: "font-family:var(--sn-mono);font-size:9px;color:var(--sn-text-dimmer);",
                                "by {persona}"
                            }
                        }
                        button {
                            class: "sn-hud-modal-close",
                            onclick: move |_| modal_open.set(false),
                            "✕"
                        }
                    }

                    // Confidence section
                    div { class: "sn-hud-modal-section",
                        div { class: "sn-hud-modal-section-hdr", "CONFIDENCE SCORE" }
                        div { class: "sn-conf-row",
                            span { class: "sn-conf-label", "Overall" }
                            div { class: "sn-conf-track",
                                div {
                                    class: "sn-conf-fill {conf_cls}",
                                    style: "width:{confidence_pct}%"
                                }
                            }
                            span { class: "sn-conf-val {conf_cls}", "{confidence_pct}%" }
                        }
                        if !props.pipeline_steps.is_empty() {
                            div { class: "sn-hud-conf-flow",
                                for step in props.pipeline_steps.iter() {
                                    {
                                        let node_cls = if step.confidence_delta > 0.0 { "pos" }
                                            else if step.confidence_delta < 0.0 { "neg" }
                                            else { "neutral" };
                                        let label = step_label(&step.step_type);
                                        let delta_str = if step.confidence_delta != 0.0 {
                                            format!("{:+.2}", step.confidence_delta)
                                        } else {
                                            "±0".to_string()
                                        };
                                        rsx! {
                                            div { class: "sn-hud-conf-node-wrap", title: "{label}: {delta_str}",
                                                div { class: "sn-hud-conf-node {node_cls}" }
                                                span { class: "sn-hud-conf-node-label",
                                                    "{step_abbrev(&step.step_type)}"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Sources section
                    if !props.sources.is_empty() {
                        div { class: "sn-hud-modal-section",
                            div { class: "sn-hud-modal-section-hdr", "SOURCES ({total_sources})" }
                            div { class: "sn-hud-sources",
                                for source in props.sources.iter() {
                                    div { class: "sn-source-item",
                                        div {
                                            class: if source.verified { "sn-source-dot ok" } else { "sn-source-dot no" }
                                        }
                                        span { style: "font-family:var(--sn-mono);font-size:10px;color:var(--sn-text-dim);flex:1;",
                                            "{source.name}"
                                        }
                                        if source.verified {
                                            span { class: "sn-gen-pill tokens",
                                                style: "font-size:7px;padding:1px 5px;margin-left:8px;",
                                                "verified"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Pipeline trail section
                    if !props.pipeline_steps.is_empty() {
                        div { class: "sn-hud-modal-section",
                            div { class: "sn-hud-modal-section-hdr", "PIPELINE ({step_count} steps)" }
                            div { class: "sn-trail", style: "padding:12px 0 4px;",
                                for (i, step) in props.pipeline_steps.iter().enumerate() {
                                    {
                                        let is_last = i == step_count - 1;
                                        let label = step_label(&step.step_type);
                                        let delta_cls = if step.confidence_delta > 0.0 { "sn-trail-delta pos" }
                                            else if step.confidence_delta < 0.0 { "sn-trail-delta neg" }
                                            else { "sn-trail-delta" };
                                        rsx! {
                                            div { class: if is_last { "sn-trail-step last" } else { "sn-trail-step" },
                                                div { class: "sn-trail-gutter",
                                                    div { class: "sn-trail-marker", "{i + 1}" }
                                                    if !is_last {
                                                        div { class: "sn-trail-line" }
                                                    }
                                                }
                                                div { class: "sn-trail-content",
                                                    div { class: "sn-trail-header",
                                                        span { class: "sn-trail-label", "{label}" }
                                                    }
                                                    if step.confidence_delta != 0.0 {
                                                        div { class: "sn-trail-meta",
                                                            span { class: "{delta_cls}",
                                                                "Δ {step.confidence_delta:+.2}"
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // AI monologue section
                    if let Some(monologue) = &props.ai_monologue {
                        {
                            let rendered = simple_md_to_html(monologue);
                            let persona_label = props.persona_name.as_deref().unwrap_or("AI");
                            rsx! {
                                div { class: "sn-hud-modal-section",
                                    div { class: "sn-hud-modal-section-hdr", "AI REASONING" }
                                    div { class: "sn-monologue",
                                        div { class: "sn-monologue-label",
                                            "INTERNAL REASONING · {persona_label}"
                                        }
                                        div { dangerous_inner_html: "{rendered}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn conf_color_class(score: f64) -> &'static str {
    if score >= 0.80 { "g" } else if score >= 0.60 { "a" } else { "r" }
}

fn step_abbrev(step_type: &str) -> &'static str {
    match step_type {
        "scan"         => "SC",
        "source_check" => "SR",
        "fact_check"   => "FC",
        "draft"        => "DR",
        "verify"       => "VF",
        "edit"         => "ED",
        _              => "?",
    }
}

fn step_label(step_type: &str) -> &'static str {
    match step_type {
        "scan"         => "DISCOVER",
        "source_check" => "SOURCE CHECK",
        "fact_check"   => "FACT CHECK",
        "draft"        => "DRAFT",
        "verify"       => "VERIFICATION",
        "edit"         => "EDITORIAL REVIEW",
        _              => "STEP",
    }
}
