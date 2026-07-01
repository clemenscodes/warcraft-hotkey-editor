pub mod components;
mod logic;
mod props;
mod style;

use crate::assert_component;
use components::collision_breadcrumb_count::{
    CollisionBreadcrumbCount, CollisionBreadcrumbCountProps,
};
use components::collision_breadcrumb_label::CollisionBreadcrumbLabel;
use dioxus::prelude::*;
use logic::CollisionBreadcrumbModel;
pub use props::CollisionBreadcrumbProps;
use style::CLASS;
assert_component!(CollisionBreadcrumb);

/// A breadcrumb tab: a label with its live collision count that navigates the
/// collisions view to its kind when clicked. Highlighted when active.
#[component]
pub fn CollisionBreadcrumb(props: CollisionBreadcrumbProps) -> Element {
    let model = CollisionBreadcrumbModel::from(&props);
    let active = props.active;
    let data_breadcrumb = props.data_breadcrumb;
    let label = props.label;
    let count = CollisionBreadcrumbCountProps { count: props.count };
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            "data-active": active,
            "aria-current": model.aria_current,
            "data-breadcrumb": data_breadcrumb,
            onclick: model.onclick,
            CollisionBreadcrumbLabel { text: label }
            CollisionBreadcrumbCount { ..count }
        }
    }
}
