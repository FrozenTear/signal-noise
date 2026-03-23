use dioxus::prelude::*;
use crate::pages::Route;

#[component]
pub fn Nav() -> Element {
    rsx! {
        nav { class: "border-b border-gray-200 mb-8",
            div { class: "max-w-5xl mx-auto px-4 py-3 flex items-center gap-6",
                Link { class: "font-bold text-lg", to: Route::Home {}, "Signal Noise" }
                div { class: "text-xs text-yellow-600 bg-yellow-50 px-2 py-0.5 rounded",
                    "AI-generated content"
                }
                div { class: "flex-1" }
                Link { class: "text-sm hover:underline", to: Route::BeatLinux {}, "Linux" }
                Link { class: "text-sm hover:underline", to: Route::BeatTech {}, "Tech" }
                Link { class: "text-sm hover:underline", to: Route::BeatPrivacy {}, "Privacy" }
                Link { class: "text-sm hover:underline", to: Route::About {}, "About" }
            }
        }
    }
}
