pub mod components;
mod data;
mod hooks;
pub(crate) mod logic;
mod model;
mod props;

use components::body::Body;
use components::breadcrumbs::Breadcrumbs;
use components::collisions_shell::CollisionsShell;
use dioxus::prelude::*;
use hooks::use_collisions_page;
pub use props::CollisionsPageProps;

/// Top-level Collisions page. Each kind renders a sidebar (islands, hotkey units,
/// or per-unit positions) beside a detail pane, under a breadcrumb bar that swaps
/// the active kind. Empty and all-clear states replace the two-pane content when
/// there is no file or no conflicts.
use tw_macro::assert_component;
assert_component!(CollisionsPage);
#[component]
pub fn CollisionsPage(props: CollisionsPageProps) -> Element {
    let model = use_collisions_page(&props);
    rsx! {
        CollisionsShell {
            Breadcrumbs { ..model.breadcrumbs }
            Body { content: model.content }
        }
    }
}
