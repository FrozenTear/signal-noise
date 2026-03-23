use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ConfidenceMeterProps {
    pub score: f64,
}

#[component]
pub fn ConfidenceMeter(props: ConfidenceMeterProps) -> Element {
    let pct = (props.score * 100.0) as u32;
    let fill_color = if props.score >= 0.8 {
        "var(--sn-accent)"
    } else if props.score >= 0.5 {
        "var(--sn-amber)"
    } else {
        "var(--sn-red)"
    };

    rsx! {
        div { class: "sn-conf-row", style: "margin-bottom:12px;",
            span { class: "sn-conf-label", "CONFIDENCE" }
            div { class: "sn-conf-track",
                div {
                    class: "sn-conf-fill",
                    style: "width:{pct}%; background:{fill_color};"
                }
            }
            span { class: "sn-conf-val", "{pct}%" }
        }
    }
}
