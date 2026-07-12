mod model;
mod view;

pub use view::HotkeyAltPositionPickerExplainerView;
mod style;

use dioxus::prelude::*;

use style::CLASS;
use tw_macro::assert_component;

use model::HotkeyAltPositionPickerExplainerModel;

/// The instruction line at the top of a position-picker dialog.
#[component]
pub fn HotkeyAltPositionPickerExplainer(props: HotkeyAltPositionPickerExplainerModel) -> Element {
    let text = props.text;
    rsx! {
        p { class: CLASS, {text} }
    }
}

assert_component!(HotkeyAltPositionPickerExplainer);
