mod model;
mod view;

pub use view::ToastDescriptionView;
mod style;

use dioxus::prelude::*;
use model::ToastDescriptionModel;
use style::CLASS;
use tw_macro::assert_component;

/// The optional secondary line under the title. Renders nothing when the toast
/// carries no description.
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
