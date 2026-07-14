pub mod components;
mod model;
mod view;

pub use view::HotkeyOverrideHeaderView;
mod style;

use dioxus::prelude::*;

use components::current_hotkey_slot::CurrentHotkeySlot;
use components::hotkey_override_header_text::HotkeyOverrideHeaderText;
use style::CLASS;
use tw_macro::assert_component;

use model::HotkeyOverrideHeaderModel;

/// The header row of the hotkey-override section: the name/id column and the hotkey slot.
#[component]
pub fn HotkeyOverrideHeader(props: HotkeyOverrideHeaderModel) -> Element {
    let HotkeyOverrideHeaderModel {
        name_text,
        object_id,
        show_hotkey_field,
        hotkey_label,
        hotkey_is_editing,
        hotkey_is_special,
        on_hotkey_activate,
        show_research_field,
        research_label,
        research_is_editing,
        research_is_special,
        on_research_activate,
        is_info_only,
    } = props;
    rsx! {
        div {
            class: CLASS,
            HotkeyOverrideHeaderText {
                name_text,
                object_id,
            }
            CurrentHotkeySlot {
                show_hotkey_field,
                hotkey_label,
                hotkey_is_editing,
                hotkey_is_special,
                on_hotkey_activate,
                show_research_field,
                research_label,
                research_is_editing,
                research_is_special,
                on_research_activate,
                is_info_only,
            }
        }
    }
}

assert_component!(HotkeyOverrideHeader);
