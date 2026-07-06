pub mod components;
mod logic;
mod props;
mod style;

use components::breadcrumb_count::{BreadcrumbCount, BreadcrumbCountProps};
use components::breadcrumb_label::BreadcrumbLabel;
use dioxus::prelude::*;
use logic::BreadcrumbModel;
pub use props::BreadcrumbProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(Breadcrumb);

/// A breadcrumb tab: a label with its live collision count that navigates the
/// collisions view to its kind when clicked. Highlighted when active.
#[component]
pub fn Breadcrumb(props: BreadcrumbProps) -> Element {
    let model = BreadcrumbModel::from(&props);
    let active = props.active;
    let data_breadcrumb = props.data_breadcrumb;
    let label = props.label;
    let count = BreadcrumbCountProps { count: props.count };
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            "data-active": active,
            "aria-current": model.aria_current,
            "data-breadcrumb": data_breadcrumb,
            onclick: model.onclick,
            BreadcrumbLabel { text: label }
            BreadcrumbCount { ..count }
        }
    }
}
