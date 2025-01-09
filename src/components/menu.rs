use crate::routes::Route;

use dioxus::html::geometry::PixelsRect;
use dioxus::prelude::*;

#[component]
pub fn Menu() -> Element {
    let mut menu_button_visible = use_signal(|| false);

    let update_memu_button_visibility = move |event: Event<VisibleData>| {
        if let Ok(bounds) = event.data.get_root_bounds() {
            menu_button_visible.set(bounds != PixelsRect::default());
        }
    };

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/css/menu.css") }
        div {
            button {
                class: "menu-button",
                popovertarget: "menu-nav",
                onvisible: update_memu_button_visibility,
                tabindex: "0",
                img {
                    class: "menu-icon",
                    src: asset!("/assets/images/icon-menu.png"),
                    alt: "Site menu toggle",
                }
            }
            nav {
                id: "menu-nav",
                popover: if menu_button_visible() { "auto" },
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
}
