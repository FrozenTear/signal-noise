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
        div { class: "border-t border-gray-200 mt-8 pt-4",
            h3 { class: "text-sm font-semibold text-gray-700 mb-3", "Editorial Trail" }
            if props.steps.is_empty() {
                p { class: "text-sm text-gray-400", "No pipeline steps recorded." }
            } else {
                ol { class: "space-y-3",
                    for step in &props.steps {
                        li { class: "flex gap-3 text-sm",
                            div { class: "w-24 shrink-0 text-gray-400 text-xs pt-0.5", "{step.completed_at}" }
                            div {
                                div { class: "font-medium", "{step.agent_name}" }
                                div { class: "text-gray-600", "{step.output_summary}" }
                                div { class: "text-xs text-gray-400",
                                    "confidence Δ {step.confidence_delta:+.2}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
