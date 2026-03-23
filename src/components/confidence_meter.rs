use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ConfidenceMeterProps {
    pub score: f64,
}

#[component]
pub fn ConfidenceMeter(props: ConfidenceMeterProps) -> Element {
    let pct = (props.score * 100.0) as u32;
    let color = if props.score >= 0.8 {
        "bg-green-500"
    } else if props.score >= 0.5 {
        "bg-yellow-500"
    } else {
        "bg-red-500"
    };

    rsx! {
        div { class: "flex items-center gap-2 my-2",
            span { class: "text-xs text-gray-500 w-24", "Confidence" }
            div { class: "flex-1 h-2 bg-gray-200 rounded-full overflow-hidden",
                div {
                    class: "h-full {color} rounded-full",
                    style: "width: {pct}%"
                }
            }
            span { class: "text-xs font-mono w-8 text-right", "{pct}%" }
        }
    }
}
