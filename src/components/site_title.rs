use dioxus::prelude::*;

#[component]
pub fn SiteTitle() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/css/title.css") }
        span {
           class: "title",
           "Nigel Eke - Personal Website"
        }
    }
}
