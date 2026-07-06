mod props;
mod style;

use dioxus::prelude::*;

use style::CLASS;
use tw_macro::assert_component;

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
