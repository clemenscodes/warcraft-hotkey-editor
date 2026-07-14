mod model;
mod view;

pub use view::HotkeyAltPositionPickerExplainerView;
mod style;

use dioxus::prelude::*;

use style::CLASS;
use tw_macro::assert_component;

use model::HotkeyAltPositionPickerExplainerModel;

#[component]
pub fn HotkeyAltPositionPickerExplainer(props: HotkeyAltPositionPickerExplainerModel) -> Element {
    let text = props.text;
    rsx! {
        p {
            class: CLASS,
            {text}
        }
    }
}

assert_component!(HotkeyAltPositionPickerExplainer);
