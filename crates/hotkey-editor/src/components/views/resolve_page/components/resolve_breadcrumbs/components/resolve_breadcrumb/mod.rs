pub mod components;
mod logic;
mod props;
mod style;
use components::resolve_breadcrumb_count::{ResolveBreadcrumbCount, ResolveBreadcrumbCountProps};
use components::resolve_breadcrumb_label::ResolveBreadcrumbLabel;
use crate::assert_component;
use dioxus::prelude::*;
use logic::ResolveBreadcrumbModel;
pub use props::ResolveBreadcrumbProps;
use style::CLASS;
assert_component!(ResolveBreadcrumb);

/// A move-category breadcrumb: a title with its move count that selects its
/// section when clicked. Highlighted when active.
#[component]
pub fn ResolveBreadcrumb(props: ResolveBreadcrumbProps) -> Element {
    let model = ResolveBreadcrumbModel::from(&props);
    let active = props.active;
    let data_breadcrumb = props.data_breadcrumb;
    let title = props.title;
    let onclick = props.onclick;
    let count = ResolveBreadcrumbCountProps { count: props.count };
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            "data-active": active,
            "aria-current": model.aria_current,
            "data-breadcrumb": data_breadcrumb,
            onclick,
            ResolveBreadcrumbLabel { text: title }
            ResolveBreadcrumbCount { ..count }
        }
    }
}
