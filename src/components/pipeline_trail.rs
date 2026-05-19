use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct PipelineStepSummary {
    pub agent_name: String,
    pub step_type: String,
    pub output_summary: String,
    pub confidence_delta: f64,
    pub completed_at: String,
    pub sort_order: i32,
}

fn step_label(step_type: &str) -> &'static str {
    match step_type {
        "scan" => "DISCOVER",
        "source_check" => "SOURCE CHECK",
        "fact_check" => "FACT CHECK",
        "draft" => "DRAFT",
        "verify" => "VERIFICATION",
        "edit" => "EDITORIAL REVIEW",
        _ => "STEP",
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct PipelineTrailProps {
    pub steps: Vec<PipelineStepSummary>,
}

#[component]
pub fn PipelineTrail(props: PipelineTrailProps) -> Element {
    let total = props.steps.len();

    rsx! {
        div { style: "margin-top:32px;",
            div { class: "sn-section-hdr",
                span { class: "hi", "EDITORIAL TRAIL" }
            }
            div { class: "sn-sb-card",
                if props.steps.is_empty() {
                    div { style: "padding:14px 16px; font-family:var(--sn-mono); font-size:10px; color:var(--sn-text-dimmer);",
                        "No pipeline steps recorded."
                    }
                } else {
                    div { class: "sn-trail",
                        for (i, step) in props.steps.iter().enumerate() {
                            {
                                let is_last = i == total - 1;
                                let delta_class = if step.confidence_delta > 0.0 { "sn-trail-delta pos" }
                                    else if step.confidence_delta < 0.0 { "sn-trail-delta neg" }
                                    else { "sn-trail-delta" };
                                rsx! {
                                    div { class: if is_last { "sn-trail-step last" } else { "sn-trail-step" },
                                        // Timeline gutter: marker + line
                                        div { class: "sn-trail-gutter",
                                            div { class: "sn-trail-marker",
                                                "{i + 1}"
                                            }
                                            if !is_last {
                                                div { class: "sn-trail-line" }
                                            }
                                        }
                                        // Content
                                        div { class: "sn-trail-content",
                                            div { class: "sn-trail-header",
                                                span { class: "sn-trail-label", "{step_label(&step.step_type)}" }
                                                span { class: "sn-trail-agent", "{step.agent_name}" }
                                            }
                                            div { class: "sn-trail-summary", "{step.output_summary}" }
                                            div { class: "sn-trail-meta",
                                                if step.confidence_delta != 0.0 {
                                                    span { class: "{delta_class}",
                                                        "Δ {step.confidence_delta:+.2}"
                                                    }
                                                }
                                                span { class: "sn-trail-ts", "{step.completed_at}" }
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
}
