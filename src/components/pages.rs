use crate::components::centered::Centered;
use crate::components::socials::Socials;
use crate::components::Markdown;
use crate::components::Template;

use crate::markdown_page;

use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        Template {
            Socials { }
        }
    }
}

#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
    rsx! {
        Template {
            Centered { "not found" }
        }
    }
}

markdown_page!(Italian, "../../markdown/italian.md");
markdown_page!(Projects, "../../markdown/projects.md");
markdown_page!(Reference, "../../markdown/reference.md");
markdown_page!(Apis, "../../markdown/apis.md");
markdown_page!(UsefulInfo, "../../markdown/useful_info.md");
