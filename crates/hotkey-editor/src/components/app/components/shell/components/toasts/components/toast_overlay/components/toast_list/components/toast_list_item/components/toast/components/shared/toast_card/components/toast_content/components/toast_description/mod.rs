mod model;
mod view;

pub use view::ToastDescriptionView;
mod style;

use dioxus::prelude::*;
use model::ToastDescriptionModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn ToastDescription(props: ToastDescriptionModel) -> Element {
    let Some(description) = props.description else {
        return rsx! {};
    };
    rsx! {
        div {
            class: CLASS,
            {description}
        }
    }
}

assert_component!(ToastDescription);
