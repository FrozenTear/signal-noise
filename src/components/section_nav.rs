use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SectionNavProps {
    pub categories: &'static [(&'static str, &'static str)],
    pub active: Option<String>,
    pub on_select: EventHandler<Option<String>>,
}

#[component]
pub fn SectionNav(props: SectionNavProps) -> Element {
    rsx! {
        nav { class: "sn-section-nav",
            for (label, value) in props.categories {
                {
                    let is_active = match &props.active {
                        None => value.is_empty(),
                        Some(a) => a == value,
                    };
                    let cls = if is_active {
                        "sn-section-nav-item active"
                    } else {
                        "sn-section-nav-item"
                    };
                    let val: &'static str = value;
                    rsx! {
                        button {
                            class: "{cls}",
                            onclick: move |_| {
                                if val.is_empty() {
                                    props.on_select.call(None);
                                } else {
                                    props.on_select.call(Some(val.to_string()));
                                }
                            },
                            "{label}"
                        }
                    }
                }
            }
        }
    }
}
