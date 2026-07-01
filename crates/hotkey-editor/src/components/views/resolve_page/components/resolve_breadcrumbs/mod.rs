pub mod components;
mod props;
mod style;
use crate::assert_component;
use components::resolve_breadcrumb::ResolveBreadcrumb;
use components::resolve_breadcrumb_separator::ResolveBreadcrumbSeparator;
use dioxus::prelude::*;
pub use props::ResolveBreadcrumbsProps;
use style::CLASS;
assert_component!(ResolveBreadcrumbs);

/// The move-category breadcrumb bar: one tab per section, separated by "|".
#[component]
pub fn ResolveBreadcrumbs(props: ResolveBreadcrumbsProps) -> Element {
    let breadcrumbs = props.breadcrumbs;
    rsx! {
        nav {
            class: CLASS,
            aria_label: "Move categories",
            for (index, breadcrumb) in breadcrumbs.into_iter().enumerate() {
                if index > 0 {
                    ResolveBreadcrumbSeparator {}
                }
                ResolveBreadcrumb { ..breadcrumb }
            }
        }
    }
}
