use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=markdown");

    let markdown_pages_rs = Path::new("src/generated/markdown_pages.rs");

    let mut new_content = String::from(
        r#"use crate::components::{Markdown, Template};
use crate::markdown_page;
use dioxus::prelude::*;

"#,
    );

    let entries = fs::read_dir("markdown").unwrap();
    let mut stems = entries
        .filter_map(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();

            path.extension().and_then(|s| {
                (s.to_str() == Some("md"))
                    .then(|| path.file_stem().unwrap().to_str().unwrap().to_string())
            })
        })
        .collect::<Vec<String>>();
    stems.sort();

    stems.iter().for_each(|stem| {
        let component = to_pascal_case(stem);
        new_content.push_str(&format!(
            r#"markdown_page!({}, "../../markdown/{}.md");
"#,
            component, stem
        ));
    });

    if let Ok(current_content) = fs::read_to_string(markdown_pages_rs) {
        if new_content != current_content {
            println!("cargo:warning=3");
            println!("cargo:warning=new {new_content:?}");
            println!("cargo:warning=cur {current_content:?}");
            fs::write(markdown_pages_rs, new_content).unwrap();
        }
    } else {
        fs::write(markdown_pages_rs, new_content).unwrap();
    }
}

fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut c = word.chars();

            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect()
}
