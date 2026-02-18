use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=markdown");

    let out_file = Path::new("src/generated/markdown_pages.rs");

    let mut output = String::from(
        r#"use crate::components::{Markdown, Template};
use crate::markdown_page;
use dioxus::prelude::*;

"#,
    );

    let entries = fs::read_dir("markdown").unwrap();

    for entry in entries {
        let entry = entry.unwrap();

        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("md") {
            continue;
        }

        let stem = path.file_stem().unwrap().to_str().unwrap();
        let component = to_pascal_case(stem);
        output.push_str(&format!(
            r#"markdown_page!({}, "../../markdown/{}.md");
"#,
            component, stem
        ));
    }

    fs::write(out_file, output).unwrap();
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
