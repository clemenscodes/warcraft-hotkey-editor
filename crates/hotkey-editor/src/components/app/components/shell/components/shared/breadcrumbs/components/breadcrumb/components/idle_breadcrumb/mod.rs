mod props;
mod style;

use super::shared::breadcrumb_count::{BreadcrumbCount, BreadcrumbCountProps};
use super::shared::breadcrumb_label::{BreadcrumbLabel, BreadcrumbLabelProps};
use dioxus::prelude::*;
pub use props::IdleBreadcrumbProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(IdleBreadcrumb);

/// The idle breadcrumb tab: a muted crumb that lightens on hover. It publishes
/// `--count-opacity: 0.8` for its dimmed count. Presentational — the dispatcher renders
/// it for every tab that is not the current page.
#[component]
pub fn IdleBreadcrumb(props: IdleBreadcrumbProps) -> Element {
    let label = BreadcrumbLabelProps { text: props.label };
    let count = BreadcrumbCountProps { count: props.count };
    let onclick = props.onclick;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            "aria-current": "false",
            onclick,
            BreadcrumbLabel { ..label }
            BreadcrumbCount { ..count }
        }
    }
}
