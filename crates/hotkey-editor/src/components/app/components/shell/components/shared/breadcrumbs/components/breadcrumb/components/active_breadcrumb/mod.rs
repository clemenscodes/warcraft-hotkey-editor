mod model;
mod view;

pub use view::ActiveBreadcrumbView;
mod style;

use super::shared::breadcrumb_count::BreadcrumbCount;
use super::shared::breadcrumb_label::BreadcrumbLabel;
use dioxus::prelude::*;
use model::ActiveBreadcrumbModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn ActiveBreadcrumb(props: ActiveBreadcrumbModel) -> Element {
    let text = props.label;
    let count = props.count;
    let onclick = props.onclick;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            "aria-current": "page",
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

assert_component!(ActiveBreadcrumb);
