use crate::components::{Apis, Home, Italian, NotFound, Projects, Reference, UsefulInfo};

use dioxus::prelude::*;

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[route("/")]
    Home,
    #[route("/apis")]
    Apis,
    #[route("/italian")]
    Italian,
    #[route("/projects")]
    Projects,
    #[route("/reference")]
    Reference,
    #[route("/useful-info")]
    UsefulInfo,
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}
