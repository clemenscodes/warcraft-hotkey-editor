mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

pub use props::AltPositionPickerExplainerProps;

assert_component!(AltPositionPickerExplainer);

/// The instruction line at the top of a position-picker dialog.
#[component]
pub fn AltPositionPickerExplainer(props: AltPositionPickerExplainerProps) -> Element {
    let text = props.text;
    rsx! {
        p { class: CLASS, {text} }
    }
}
