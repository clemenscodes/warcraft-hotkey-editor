mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

pub use props::AltPositionPickerBodyProps;

assert_component!(AltPositionPickerBody);

/// The centered body of a position-picker dialog.
#[component]
pub fn AltPositionPickerBody(props: AltPositionPickerBodyProps) -> Element {
    let children = props.children;
    rsx! {
        div { class: CLASS, {children} }
    }
}
