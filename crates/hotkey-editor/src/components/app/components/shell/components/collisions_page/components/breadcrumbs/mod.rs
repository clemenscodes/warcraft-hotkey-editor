pub mod components;
mod data;
mod logic;
mod props;
mod style;

use components::breadcrumb::Breadcrumb;
use components::breadcrumb_separator::BreadcrumbSeparator;
use dioxus::prelude::*;
use logic::BreadcrumbsModel;
pub use props::BreadcrumbsProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(Breadcrumbs);

/// The breadcrumb bar under the header: one tab per collision kind — cross-unit
/// position, per-unit position, and hotkey — each with its live count. Clicking a
/// tab swaps the view below.
#[component]
pub fn Breadcrumbs(props: BreadcrumbsProps) -> Element {
    let model = BreadcrumbsModel::from(&props);
    rsx! {
        nav {
            class: CLASS,
            aria_label: "Collision categories",
            Breadcrumb { ..model.positions }
            BreadcrumbSeparator {}
            Breadcrumb { ..model.unit_positions }
            BreadcrumbSeparator {}
            Breadcrumb { ..model.hotkeys }
        }
    }
}
