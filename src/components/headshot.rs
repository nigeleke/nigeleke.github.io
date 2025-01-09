use dioxus::prelude::*;

#[component]
pub fn HeadShot() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/css/headshot.css") }
        img {
            class: "headshot",
            src: asset!("/assets/images/nigeleke-notion.png"),
            alt: "Cartoonesque image",
        }
    }
}
