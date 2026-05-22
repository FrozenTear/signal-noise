use dioxus::prelude::*;

const REGIONS: &[(&str, &str)] = &[
    ("All regions", ""),
    ("American",    "american"),
    ("European",    "european"),
    ("Global",      "global"),
];

#[derive(Props, Clone, PartialEq)]
pub struct SectionNavProps {
    pub categories: Vec<(String, String)>,
    pub active: Option<String>,
    pub on_select: EventHandler<Option<String>>,
    #[props(default)]
    pub active_region: Option<String>,
    pub on_region_select: EventHandler<Option<String>>,
}

#[component]
pub fn SectionNav(props: SectionNavProps) -> Element {
    let on_select = props.on_select;
    let on_region_select = props.on_region_select;
    let active = props.active.clone();
    let active_region = props.active_region.clone();

    rsx! {
        nav { class: "sn-section-nav",
            // Category row
            div { class: "sn-section-nav-row",
                for (label, value) in props.categories {
                    {
                        let is_active = match &active {
                            None => value.is_empty(),
                            Some(a) => a == &value,
                        };
                        let cls = if is_active {
                            "sn-section-nav-item active"
                        } else {
                            "sn-section-nav-item"
                        };
                        let val = value.clone();
                        rsx! {
                            button {
                                key: "{val}",
                                class: "{cls}",
                                onclick: move |_| {
                                    if val.is_empty() {
                                        on_select.call(None);
                                    } else {
                                        on_select.call(Some(val.clone()));
                                    }
                                },
                                "{label}"
                            }
                        }
                    }
                }
            }
            // Region filter row
            div { class: "sn-section-nav-row sn-region-row",
                for (label, value) in REGIONS {
                    {
                        let is_active = match &active_region {
                            None => value.is_empty(),
                            Some(r) => r == value,
                        };
                        let cls = if is_active {
                            "sn-region-item active"
                        } else {
                            "sn-region-item"
                        };
                        let val: &'static str = value;
                        rsx! {
                            button {
                                class: "{cls}",
                                onclick: move |_| {
                                    if val.is_empty() {
                                        on_region_select.call(None);
                                    } else {
                                        on_region_select.call(Some(val.to_string()));
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
}
