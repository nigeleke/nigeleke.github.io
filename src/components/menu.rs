use crate::routes::Route;

use dioxus::prelude::*;

#[component]
pub fn Menu() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/css/menu.css") }
        nav {
            Link { to: Route::Home {},
                "home"
            }
            Link { to: Route::Italian {},
                "italian"
            }
            Link { to: Route::Projects {},
                "projects"
            }
            Link { to: Route::Reference {},
                "reference"
            }
            Link { to: Route::Apis {},
                "apis"
            }
            Link { to: Route::UsefulInfo {},
                "useful info"
            }
        }
    }
}
