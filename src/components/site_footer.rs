use super::external_link::ExternalLink;

use dioxus::prelude::*;

#[component]
pub fn SiteFooter() -> Element {
    rsx! {
        span { "Copyright © 2025-2026; Nigel Eke. All rights reserved." }
        span { "Made with " ExternalLink { href: "https://dioxuslabs.com/", "Dioxus" } }
    }
}
