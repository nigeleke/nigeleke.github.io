#[macro_export]
macro_rules! markdown_page {
    ($id:ident, $path:literal) => {
        #[component]
        pub fn $id() -> Element {
            let camel_case_with_spaces = |s: &str| {
                let mut result = String::new();
                for (i, c) in s.chars().enumerate() {
                    if c.is_uppercase() && i > 0 {
                        result.push(' ');
                    }
                    result.push(c);
                }
                result
            };
            let title = camel_case_with_spaces(stringify!($id));
            rsx! {
                Template {
                    h1 { {title} }
                    Markdown {
                        markdown: include_str!($path)
                    }
                }
            }
        }
    };
}
