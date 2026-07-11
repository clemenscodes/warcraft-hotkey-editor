pub mod components;
mod model;
mod view;

pub use view::TileOverrideHeaderView;
mod style;

use dioxus::prelude::*;

use components::tile_override_header_text::TileOverrideHeaderText;
use components::tile_override_hotkey_slot::TileOverrideHotkeySlot;
use style::CLASS;
use tw_macro::assert_component;

use model::TileOverrideHeaderModel;

/// The header row of the override panel: the name/id column and the hotkey slot.
#[component]
pub fn TileOverrideHeader(props: TileOverrideHeaderModel) -> Element {
    let TileOverrideHeaderModel {
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
        div { class: CLASS,
            TileOverrideHeaderText { name_text, object_id }
            TileOverrideHotkeySlot {
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

assert_component!(TileOverrideHeader);
