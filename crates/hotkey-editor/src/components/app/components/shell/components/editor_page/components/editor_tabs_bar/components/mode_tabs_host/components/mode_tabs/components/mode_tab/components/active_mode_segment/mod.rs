mod model;
mod view;

pub use view::ActiveModeSegmentView;
mod style;

use dioxus::prelude::*;
use model::ActiveModeSegmentModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn ActiveModeSegment(props: ActiveModeSegmentModel) -> Element {
    let label = props.label;
    let onclick = props.onclick;
    let onkeydown = props.onkeydown;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            aria_pressed: true,
            onclick,
            onkeydown,
            {label}
        }
    }
}

assert_component!(ActiveModeSegment);
