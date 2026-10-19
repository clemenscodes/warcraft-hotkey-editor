pub mod components;
mod model;
mod presentation;
mod view;

pub use view::HotkeyOverrideView;
mod state;

use dioxus::prelude::*;

use components::hotkey_alt_position_picker_dialog::HotkeyAltPositionPickerDialog;
use components::hotkey_override_card::HotkeyOverrideCard;
use components::hotkey_picker_dialog::HotkeyPickerDialog;
use components::hotkey_upgrade_position_picker_dialog::HotkeyUpgradePositionPickerDialog;
use model::{HotkeyOverrideModel, HotkeyOverridePresentation};
use presentation::use_hotkey_override;
use tw_macro::assert_component;

#[component]
pub fn HotkeyOverride(props: HotkeyOverrideModel) -> Element {
    let HotkeyOverridePresentation {
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
        alt_name_text,
        show_alt_controls,
        alt_hotkey_label,
        alt_hotkey_is_editing,
        alt_hotkey_is_special_token,
        on_hotkey_alt_position_click,
        on_alt_hotkey_activate,
        upgrade_show,
        upgrade_hotkey_label,
        upgrade_is_editing,
        upgrade_hotkey_is_special,
        on_hotkey_upgrade_position_click,
        on_upgrade_hotkey_activate,
        show_tier_selector,
        tier_label,
        on_previous_tier,
        on_next_tier,
        key_picker_visible,
        picker_title,
        picker_rows,
        on_pick,
        on_close,
        alt_display_name,
        alt_picker_slots,
        alt_open,
        on_alt_open_change,
        upgrade_unit_id,
        upgrade_display_name,
        upgrade_picker_slots,
        upgrade_open,
        on_upgrade_open_change,
    } = use_hotkey_override(&props);
    rsx! {
        HotkeyOverrideCard {
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
            alt_name_text,
            show_alt_controls,
            alt_hotkey_label,
            alt_hotkey_is_editing,
            alt_hotkey_is_special_token,
            on_hotkey_alt_position_click,
            on_alt_hotkey_activate,
            upgrade_show,
            upgrade_hotkey_label,
            upgrade_is_editing,
            upgrade_hotkey_is_special,
            on_hotkey_upgrade_position_click,
            on_upgrade_hotkey_activate,
            show_tier_selector,
            tier_label,
            on_previous_tier,
            on_next_tier,
        }
        HotkeyPickerDialog {
            visible: key_picker_visible,
            title: picker_title,
            rows: picker_rows,
            on_pick,
            on_close,
        }
        HotkeyAltPositionPickerDialog {
            object_id,
            display_name: alt_display_name,
            picker_slots: alt_picker_slots,
            open: alt_open,
            on_open_change: on_alt_open_change,
        }
        HotkeyUpgradePositionPickerDialog {
            upgrade_unit_id,
            display_name: upgrade_display_name,
            picker_slots: upgrade_picker_slots,
            open: upgrade_open,
            on_open_change: on_upgrade_open_change,
        }
    }
}

assert_component!(HotkeyOverride);
