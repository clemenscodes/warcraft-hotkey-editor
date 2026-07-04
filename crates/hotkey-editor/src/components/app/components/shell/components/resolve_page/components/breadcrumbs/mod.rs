pub mod components;
mod props;
mod style;
use crate::assert_component;
use components::breadcrumb::Breadcrumb;
use components::breadcrumb_separator::BreadcrumbSeparator;
use dioxus::prelude::*;
pub use props::BreadcrumbsProps;
use style::CLASS;
assert_component!(Breadcrumbs);

/// The move-category breadcrumb bar: one tab per section, separated by "|".
#[component]
pub fn Breadcrumbs(props: BreadcrumbsProps) -> Element {
    let breadcrumbs = props.breadcrumbs;
    rsx! {
        nav {
            class: CLASS,
            aria_label: "Move categories",
            for (index, breadcrumb) in breadcrumbs.into_iter().enumerate() {
                if index > 0 {
                    BreadcrumbSeparator {}
                }
                Breadcrumb { ..breadcrumb }
            }
        }
    }
}
