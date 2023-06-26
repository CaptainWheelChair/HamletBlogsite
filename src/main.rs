use dioxus::prelude::*;
use dioxus_web;
pub mod blog;
use blog::App as blog;

pub fn main() {
    dioxus_web::launch(
        blog
    )
}
