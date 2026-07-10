mod props;
mod view;

pub use view::IdleBreadcrumbView;
mod style;

use super::shared::breadcrumb_count::BreadcrumbCount;
use super::shared::breadcrumb_label::BreadcrumbLabel;
use dioxus::prelude::*;
use props::IdleBreadcrumbProps;
use style::CLASS;
use tw_macro::assert_component;

/// The idle breadcrumb tab: a muted crumb that lightens on hover. It publishes
/// `--count-opacity: 0.8` for its dimmed count. Presentational — the dispatcher renders
/// it for every tab that is not the current page.
#[component]
pub fn IdleBreadcrumb(props: IdleBreadcrumbProps) -> Element {
    let text = props.label;
    let count = props.count;
    let onclick = props.onclick;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            "aria-current": "false",
            onclick,
            BreadcrumbLabel { text }
            BreadcrumbCount { count }
        }
    }
}

assert_component!(IdleBreadcrumb);
