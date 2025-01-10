use dioxus::prelude::*;

#[component]
pub fn ExternalLink(
    href: String,
    children: Element,
    #[props(default = false)] quiet: bool,
) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/css/external_link.css") },
        a {
            class: if !quiet { "external-link" },
            href,
            target: "_blank",
            {children}
        }
    }
}
