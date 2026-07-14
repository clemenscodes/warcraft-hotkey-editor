mod model;
mod view;

pub use view::BreadcrumbLabelView;
mod style;
use dioxus::prelude::*;
use model::BreadcrumbLabelModel;
use style::CLASS;
use tw_macro::assert_component;
#[component]
pub fn BreadcrumbLabel(props: BreadcrumbLabelModel) -> Element {
    let text = props.text;
    rsx! {
        span {
            class: CLASS,
            {text}
        }
    }
}

assert_component!(BreadcrumbLabel);
