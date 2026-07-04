mod props;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
pub use props::ToastDescriptionProps;
use style::CLASS;
assert_component!(ToastDescription);

/// The optional secondary line under the title. Renders nothing when the toast
/// carries no description.
#[component]
pub fn ToastDescription(props: ToastDescriptionProps) -> Element {
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
