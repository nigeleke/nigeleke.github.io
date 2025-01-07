use dioxus::prelude::*;
use pulldown_cmark::{Options, Parser};

#[component]
pub fn Markdown(markdown: String) -> Element {
    let options = Options::all();
    let parser = Parser::new_ext(&markdown, options);
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, parser);

    rsx! {
        div {
            dangerous_inner_html: html,
        }
    }
}
