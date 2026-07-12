pub mod components;
mod model;
mod view;

pub use view::HotkeyOverrideHeaderTextView;
mod style;

use dioxus::prelude::*;

use components::ability_id::AbilityId;
use components::ability_name::AbilityName;
use style::CLASS;
use tw_macro::assert_component;

use model::HotkeyOverrideHeaderTextModel;

/// The name-and-id column of the override panel header.
#[component]
pub fn HotkeyOverrideHeaderText(props: HotkeyOverrideHeaderTextModel) -> Element {
    let HotkeyOverrideHeaderTextModel {
        name_text,
        object_id,
    } = props;
    rsx! {
        div { class: CLASS,
            AbilityName { text: name_text }
            AbilityId { object_id }
        }
    }
}

assert_component!(HotkeyOverrideHeaderText);
