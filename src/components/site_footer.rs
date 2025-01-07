use dioxus::prelude::*;

#[component]
pub fn SiteFooter() -> Element {
    rsx! {
        span { "Copyright © 2025; Nigel Eke. All rights reserved." }
        span { "Made with " a { href: "https://dioxuslabs.com/", "Dioxus" } }
    }
}
