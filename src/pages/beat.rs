use dioxus::prelude::*;

#[component]
pub fn BeatLinux() -> Element {
    rsx! { Beat { category: "linux".to_string(), title: "Linux & Open Source".to_string() } }
}

#[component]
pub fn BeatTech() -> Element {
    rsx! { Beat { category: "tech".to_string(), title: "Technology".to_string() } }
}

#[component]
pub fn BeatPrivacy() -> Element {
    rsx! { Beat { category: "privacy".to_string(), title: "Privacy & Surveillance".to_string() } }
}

#[component]
fn Beat(category: String, title: String) -> Element {
    rsx! {
        div { class: "max-w-5xl mx-auto px-4 py-8",
            h1 { class: "text-2xl font-bold mb-6", "{title}" }
            // TODO: fetch articles for category via server function
            div { "Loading {category} articles..." }
        }
    }
}
