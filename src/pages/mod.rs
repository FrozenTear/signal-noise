pub mod about;
pub mod article;
pub mod beat;
pub mod home;

use dioxus::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/article/:slug")]
    Article { slug: String },
    #[route("/linux")]
    BeatLinux {},
    #[route("/tech")]
    BeatTech {},
    #[route("/privacy")]
    BeatPrivacy {},
    #[route("/about")]
    About {},
}
