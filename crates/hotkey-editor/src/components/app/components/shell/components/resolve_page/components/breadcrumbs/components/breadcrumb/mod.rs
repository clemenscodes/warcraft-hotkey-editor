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

/// A move-category breadcrumb: a title with its move count that selects its
/// section when clicked. Highlighted when active.
#[component]
pub fn Breadcrumb(props: BreadcrumbProps) -> Element {
    let model = BreadcrumbModel::from(&props);
    let active = props.active;
    let data_breadcrumb = props.data_breadcrumb;
    let title = props.title;
    let onclick = props.onclick;
    let count = BreadcrumbCountProps { count: props.count };
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            "data-active": active,
            "aria-current": model.aria_current,
            "data-breadcrumb": data_breadcrumb,
            onclick,
            BreadcrumbLabel { text: title }
            BreadcrumbCount { ..count }
        }
    }
}
