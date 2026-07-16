pub mod components;
mod model;
mod view;

pub use view::HotkeyAltPositionPickerBodyView;
mod style;

use components::hotkey_alt_position_picker_explainer::HotkeyAltPositionPickerExplainer;
use components::hotkey_alt_position_picker_grid_anchor::HotkeyAltPositionPickerGridAnchor;
use dioxus::prelude::*;

use style::CLASS;
use tw_macro::assert_component;

use model::HotkeyAltPositionPickerBodyModel;

#[component]
pub fn HotkeyAltPositionPickerBody(props: HotkeyAltPositionPickerBodyModel) -> Element {
    let HotkeyAltPositionPickerBodyModel {
        explainer_text,
        grid_config,
    } = props;
    rsx! {
        div {
            class: CLASS,
            HotkeyAltPositionPickerExplainer {
                text: explainer_text,
            }
            HotkeyAltPositionPickerGridAnchor {
                grid_config,
            }
        }
    }
}

assert_component!(HotkeyAltPositionPickerBody);
