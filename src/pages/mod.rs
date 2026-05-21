pub mod about;
pub mod article;
pub mod beat;
pub mod h2h;
pub mod home;

use dioxus::prelude::*;

pub use about::About;
pub use article::Article;
pub use beat::{BeatLinux, BeatPrivacy, BeatTech};
pub use h2h::H2H;
pub use home::Home;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/article/:slug")]
    Article { slug: String },
    #[route("/h2h/:slug")]
    H2H { slug: String },
    #[route("/linux")]
    BeatLinux {},
    #[route("/tech")]
    BeatTech {},
    #[route("/privacy")]
    BeatPrivacy {},
    #[route("/about")]
    About {},
}
