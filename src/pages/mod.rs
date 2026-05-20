pub mod about;
pub mod article;
pub mod beat;
pub mod head2head;
pub mod home;
pub mod rejections;

use dioxus::prelude::*;

pub use about::About;
pub use article::Article;
pub use beat::{BeatLinux, BeatPrivacy, BeatTech};
pub use head2head::Head2Head;
pub use home::Home;
pub use rejections::Rejections;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/article/:slug")]
    Article { slug: String },
    #[route("/h2h/:slug")]
    Head2Head { slug: String },
    #[route("/linux")]
    BeatLinux {},
    #[route("/tech")]
    BeatTech {},
    #[route("/privacy")]
    BeatPrivacy {},
    #[route("/rejections")]
    Rejections {},
    #[route("/about")]
    About {},
}
