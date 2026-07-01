mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

pub use props::AltPositionPickerGridAnchorProps;

assert_component!(AltPositionPickerGridAnchor);

/// Centers and picker-restyles the embedded command grid inside a position picker.
#[component]
pub fn AltPositionPickerGridAnchor(props: AltPositionPickerGridAnchorProps) -> Element {
    let children = props.children;
    rsx! {
        div { class: CLASS, {children} }
    }
}
