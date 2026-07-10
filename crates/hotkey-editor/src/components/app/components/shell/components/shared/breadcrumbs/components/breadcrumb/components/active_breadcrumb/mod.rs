mod props;
mod style;

use super::shared::breadcrumb_count::{BreadcrumbCount, BreadcrumbCountProps};
use super::shared::breadcrumb_label::{BreadcrumbLabel, BreadcrumbLabelProps};
use dioxus::prelude::*;
pub use props::ActiveBreadcrumbProps;
use style::CLASS;
use tw_macro::assert_component;

/// The active breadcrumb tab: the current page's crumb, its label and count lit gold.
/// It publishes `--count-opacity: 1` so its count reads full-strength. Presentational —
/// the dispatcher renders it for the active tab.
#[component]
pub fn ActiveBreadcrumb(props: ActiveBreadcrumbProps) -> Element {
    let label = BreadcrumbLabelProps { text: props.label };
    let count = BreadcrumbCountProps { count: props.count };
    let onclick = props.onclick;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            "aria-current": "page",
            onclick,
            BreadcrumbLabel { ..label }
            BreadcrumbCount { ..count }
        }
    }
}

assert_component!(ActiveBreadcrumb);
