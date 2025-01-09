use super::menu::Menu;
use super::site_footer::SiteFooter;
use super::site_title::SiteTitle;

use dioxus::prelude::*;

#[component]
pub fn Template(children: Element) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/css/main.css") }
        main {
            header { SiteTitle {} Menu {} }
            section {
                {children}
            }
            footer { SiteFooter {} }
        }
    }
}
