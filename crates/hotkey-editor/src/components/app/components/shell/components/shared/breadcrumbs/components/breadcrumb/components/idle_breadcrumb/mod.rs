mod model;
mod view;

pub use view::IdleBreadcrumbView;
mod style;

use super::shared::breadcrumb_count::BreadcrumbCount;
use super::shared::breadcrumb_label::BreadcrumbLabel;
use dioxus::prelude::*;
use model::IdleBreadcrumbModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn IdleBreadcrumb(props: IdleBreadcrumbModel) -> Element {
    let text = props.label;
    let count = props.count;
    let onclick = props.onclick;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            "aria-current": "false",
            onclick,
            BreadcrumbLabel {
                text,
            }
            BreadcrumbCount {
                count,
            }
        }
    }
}

assert_component!(IdleBreadcrumb);
