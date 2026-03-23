use dioxus::prelude::*;

use crate::components::confidence_meter::ConfidenceMeter;
use crate::components::source_block::SourceBlock;
use crate::components::pipeline_trail::PipelineTrail;

#[component]
pub fn Article(slug: String) -> Element {
    rsx! {
        div { class: "max-w-3xl mx-auto px-4 py-8",
            // TODO: fetch article via server function using slug
            p { "Loading article: {slug}" }
            ConfidenceMeter { score: 0.0 }
            SourceBlock { sources: vec![] }
            PipelineTrail { steps: vec![] }
        }
    }
}
