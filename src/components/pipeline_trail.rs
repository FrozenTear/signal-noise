use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct PipelineStepSummary {
    pub agent_name: String,
    pub step_type: String,
    pub output_summary: String,
    pub confidence_delta: f64,
    pub completed_at: String,
}

#[derive(Props, Clone, PartialEq)]
pub struct PipelineTrailProps {
    pub steps: Vec<PipelineStepSummary>,
}

#[component]
pub fn PipelineTrail(props: PipelineTrailProps) -> Element {
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
                    for step in &props.steps {
                        div { class: "sn-pipe-step",
                            div { class: "sn-pipe-agent", "{step.agent_name}" }
                            div { class: "sn-pipe-text",
                                "{step.output_summary}"
                                span { style: "display:block; margin-top:3px; color:var(--sn-text-dimmer);",
                                    "Δ {step.confidence_delta:+.2}  ·  {step.completed_at}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
