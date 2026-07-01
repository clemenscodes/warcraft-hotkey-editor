use dioxus::prelude::*;

pub mod app;
pub mod components;
pub mod model;
pub mod services;
pub mod styling;

/// The editor's compiled Tailwind stylesheet, exposed so a consumer (the
/// component gallery) can inject it and render the editor's components with
/// their real styling.
pub const TAILWIND_STYLES: Asset = asset!("/assets/tailwind.css");
