mod props;
mod style;

use dioxus::prelude::*;

use style::CLASS;
use tw_macro::assert_component;

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
