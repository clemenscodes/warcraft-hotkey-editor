mod logic;
mod props;
mod style;

use super::collision_breadcrumb::CollisionBreadcrumb;
use super::collision_breadcrumb_separator::CollisionBreadcrumbSeparator;
use crate::assert_component;
use dioxus::prelude::*;
use logic::CollisionBreadcrumbsModel;
pub use props::CollisionBreadcrumbsProps;
use style::CLASS;
assert_component!(CollisionBreadcrumbs);

/// The breadcrumb bar under the header: one tab per collision kind — cross-unit
/// position, per-unit position, and hotkey — each with its live count. Clicking a
/// tab swaps the view below.
#[component]
pub fn CollisionBreadcrumbs(props: CollisionBreadcrumbsProps) -> Element {
    let model = CollisionBreadcrumbsModel::from(&props);
    rsx! {
        nav {
            class: CLASS,
            aria_label: "Collision categories",
            CollisionBreadcrumb { ..model.positions }
            CollisionBreadcrumbSeparator {}
            CollisionBreadcrumb { ..model.unit_positions }
            CollisionBreadcrumbSeparator {}
            CollisionBreadcrumb { ..model.hotkeys }
        }
    }
}
