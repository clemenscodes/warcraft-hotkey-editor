mod model;
mod view;

pub use view::IdleModeSegmentView;
mod style;

use dioxus::prelude::*;
use model::IdleModeSegmentModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn IdleModeSegment(props: IdleModeSegmentModel) -> Element {
    let label = props.label;
    let onclick = props.onclick;
    let onkeydown = props.onkeydown;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            aria_pressed: false,
            onclick,
            onkeydown,
            {label}
        }
    }
}

assert_component!(IdleModeSegment);
