pub mod about;
pub mod article;
pub mod beat;
pub mod home;

use dioxus::prelude::*;

pub use about::About;
pub use article::Article;
pub use beat::{BeatLinux, BeatPrivacy, BeatTech};
pub use home::Home;

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
