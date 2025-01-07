use dioxus::prelude::*;

const FACEBOOK: Asset = asset!("/assets/images/facebook.png");
const GITHUB: Asset = asset!("/assets/images/github.png");
const LINKEDIN: Asset = asset!("/assets/images/linkedin.png");
const STACKOVERFLOW: Asset = asset!("/assets/images/stackoverflow.png");

#[component]
pub fn Socials() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/css/socials.css") }
        div {
            class: "socials",
            Social { href: "https://www.facebook.com/nigel.eke", img_src: FACEBOOK, alt: "Nigel's Facebook" }
            Social { href: "https://www.linkedin.com/in/nigeleke/", img_src: LINKEDIN, alt: "Nigel's LinkedIn" }
            Social { href: "https://github.com/nigeleke", img_src: GITHUB, alt: "Nigel's GitHub" }
            Social { href: "https://stackoverflow.com/users/1460067/nigel-eke", img_src: STACKOVERFLOW, alt: "Nigel's Stackoverflow" }
        }
    }
}

#[component]
fn Social(href: String, img_src: Asset, alt: String) -> Element {
    rsx! {
        a {
            href: href,
            target: "_blank",
            img {
                src: img_src,
                alt: alt,
                width: "30",
                height: "30",
            }
        }

    }
}
