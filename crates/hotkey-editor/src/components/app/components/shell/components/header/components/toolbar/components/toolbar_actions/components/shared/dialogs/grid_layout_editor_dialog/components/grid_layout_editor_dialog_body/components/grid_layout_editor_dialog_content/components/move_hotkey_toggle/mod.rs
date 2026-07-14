pub mod components;
mod data;
mod model;
mod view;

pub use view::MoveHotkeyToggleView;
mod style;

use components::move_hotkey_checkbox::MoveHotkeyCheckbox;
use dioxus::prelude::*;
use model::MoveHotkeyToggleModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn MoveHotkeyToggle(props: MoveHotkeyToggleModel) -> Element {
    let checked = props.checked;
    let on_toggle = props.on_toggle;
    let label = data::LABEL;
    rsx! {
        label {
            class: CLASS,
            MoveHotkeyCheckbox {
                checked,
                on_toggle,
            }
            {label}
        }
    }
}

assert_component!(MoveHotkeyToggle);
