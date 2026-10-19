mod model;
mod view;

pub use view::TopHotkeyMarkerView;
mod style;

use super::shared::conflict_hotkey_key::ConflictHotkeyKey;
use dioxus::prelude::*;
use model::TopHotkeyMarkerModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn TopHotkeyMarker(props: TopHotkeyMarkerModel) -> Element {
    let label = props.label;
    rsx! {
        span {
            class: CLASS,
            ConflictHotkeyKey {
                text: label,
            }
        }
    }
}

assert_component!(TopHotkeyMarker);
