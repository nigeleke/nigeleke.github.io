use crate::components::headshot::HeadShot;
use crate::components::socials::Socials;
use crate::components::Template;

use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        Template {
            Socials { }
            HeadShot { }
        }
    }
}

#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/css/not_found.css") }
        Template {
            div {
                class: "not-found-container",
                img {
                    class: "not-found-background-image",
                    src: asset!("/assets/images/water.jpg"),
                    alt: "water_by_viaankart CC BY-NC-ND 3.0",
                }
                div {
                    class: "not-found-overlay-text",
                    h1 { "404" }
                    p { "He's fallen in the water!" }
                }
            }
        }
    }
}
