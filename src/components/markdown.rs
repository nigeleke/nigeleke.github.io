use dioxus::prelude::*;
use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};

#[component]
pub fn Markdown(markdown: String) -> Element {
    let options = Options::all();
    let parser = Parser::new_ext(&markdown, options);

    let mut events = Vec::new();

    for event in parser {
        match event {
            Event::Start(Tag::Link {
                link_type: _,
                dest_url,
                title,
                id,
            }) => {
                events.push(Event::Html(
                    format!(
                        r#"<a class="external-link" href="{}" title="{}" id="{}" target="_blank">"#,
                        dest_url, title, id
                    )
                    .into(),
                ));
            }
            Event::End(TagEnd::Link) => {
                events.push(Event::Html("</a>".into()));
            }
            other => events.push(other),
        }
    }

    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, events.into_iter());

    rsx! {
        div {
            dangerous_inner_html: html,
        }
    }
}
