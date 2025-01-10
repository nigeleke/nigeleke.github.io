use dioxus::prelude::*;

#[component]
pub fn SiteTitle() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/css/title.css") }
        a {
           class: "title",
           href: "https://nigeleke.github.io/",
           "niGel eKe - personal site"
        }
    }
}
