use dioxus::prelude::*;

#[component]
pub fn Centered(children: Element) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("assets/css/centered.css") }
        div {
            class: "centered",
            {children}
        }
    }
}
