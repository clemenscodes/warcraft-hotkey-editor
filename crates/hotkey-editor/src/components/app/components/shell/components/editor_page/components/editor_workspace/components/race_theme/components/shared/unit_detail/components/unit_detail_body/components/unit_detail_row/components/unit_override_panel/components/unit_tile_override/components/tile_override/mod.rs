pub mod components;
mod hooks;
mod logic;
mod props;
mod view;

pub use view::TileOverrideView;
mod state;

use dioxus::prelude::*;

use components::tile_override_alt_picker::TileOverrideAltPicker;
use components::tile_override_card::TileOverrideCard;
use components::tile_override_key_picker::TileOverrideKeyPicker;
use components::tile_override_upgrade_picker::TileOverrideUpgradePicker;
use hooks::use_tile_override;
use props::{TileOverrideModel, TileOverrideProps};
use tw_macro::assert_component;

/// The per-tile override editor. A pure renderer: the composed hook shapes the card's
/// fields and the pickers as domain values, and the body names each child directly.
#[component]
pub fn TileOverride(props: TileOverrideProps) -> Element {
    let TileOverrideModel {
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
        description_lines,
        alt_name_text,
        alt_description_lines,
        show_alt_controls,
        alt_hotkey_label,
        alt_hotkey_is_editing,
        alt_hotkey_is_special_token,
        on_alt_position_click,
        on_alt_hotkey_activate,
        upgrade_show,
        upgrade_hotkey_label,
        upgrade_is_editing,
        upgrade_hotkey_is_special,
        on_upgrade_position_click,
        on_upgrade_hotkey_activate,
        active_tier_index,
        total_tier_count,
        tier_label_text,
        key_picker_visible,
        picker_title,
        picker_rows,
        on_pick,
        on_close,
        alt_display_name,
        alt_picker_slots,
        alt_open,
        upgrade_unit_id,
        upgrade_display_name,
        upgrade_picker_slots,
        upgrade_open,
    } = use_tile_override(&props);
    rsx! {
        TileOverrideCard {
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
            description_lines,
            alt_name_text,
            alt_description_lines,
            show_alt_controls,
            alt_hotkey_label,
            alt_hotkey_is_editing,
            alt_hotkey_is_special_token,
            on_alt_position_click,
            on_alt_hotkey_activate,
            upgrade_show,
            upgrade_hotkey_label,
            upgrade_is_editing,
            upgrade_hotkey_is_special,
            on_upgrade_position_click,
            on_upgrade_hotkey_activate,
            active_tier_index,
            total_tier_count,
            tier_label_text,
        }
        TileOverrideKeyPicker {
            visible: key_picker_visible,
            title: picker_title,
            rows: picker_rows,
            on_pick,
            on_close,
        }
        TileOverrideAltPicker {
            object_id,
            display_name: alt_display_name,
            picker_slots: alt_picker_slots,
            alt_position_picker_open: alt_open,
        }
        TileOverrideUpgradePicker {
            upgrade_unit_id,
            display_name: upgrade_display_name,
            picker_slots: upgrade_picker_slots,
            upgrade_position_picker_open: upgrade_open,
        }
    }
}

assert_component!(TileOverride);
