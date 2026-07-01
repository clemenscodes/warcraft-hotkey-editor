pub mod components;
mod hooks;
pub mod logic;
mod props;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
use hooks::use_resolve_page;
pub use props::ResolvePageProps;

assert_component!(ResolvePage);

/// The Resolve page: a transparent preview of the cascade plan — every move the
/// algorithm would make and any unresolved abilities — with an Apply button that
/// runs the cascade. Shows an upload prompt with no file and an all-clear state
/// when there is nothing to resolve.
#[component]
pub fn ResolvePage(props: ResolvePageProps) -> Element {
    use_resolve_page(&props)
}
