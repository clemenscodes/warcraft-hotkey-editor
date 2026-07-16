pub mod components;
mod model;
mod presentation;
mod view;

pub use view::HotkeyOverrideCardView;
mod style;

use dioxus::prelude::*;
use presentation::{HotkeyOverrideCardFit, use_hotkey_override_card_fit};

use components::ability_description::AbilityDescription;
use components::ability_tier::AbilityTier;
use components::alt_state_section::AltStateSection;
use components::hotkey_override_header::HotkeyOverrideHeader;
use components::upgrade_section::UpgradeSection;
use style::CLASS;
use tw_macro::assert_component;

use model::HotkeyOverrideCardModel;

#[component]
pub fn HotkeyOverrideCard(props: HotkeyOverrideCardModel) -> Element {
    let HotkeyOverrideCardFit {
        font_style,
        onmounted,
    } = use_hotkey_override_card_fit(&props);
    let HotkeyOverrideCardModel {
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
        on_hotkey_alt_position_click,
        on_alt_hotkey_activate,
        upgrade_show,
        upgrade_hotkey_label,
        upgrade_is_editing,
        upgrade_hotkey_is_special,
        on_hotkey_upgrade_position_click,
        on_upgrade_hotkey_activate,
        active_tier_index,
        total_tier_count,
        tier_label_text,
    } = props;
    rsx! {
        div {
            class: CLASS,
            style: font_style,
            onmounted: move |event| onmounted.call(event),
            HotkeyOverrideHeader {
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
            }
            AbilityDescription {
                description_lines,
            }
            AltStateSection {
                alt_name_text,
                alt_description_lines,
                show_alt_controls,
                alt_hotkey_label,
                alt_hotkey_is_editing,
                alt_hotkey_is_special_token,
                on_position_click: on_hotkey_alt_position_click,
                on_hotkey_activate: on_alt_hotkey_activate,
            }
            UpgradeSection {
                show: upgrade_show,
                upgrade_hotkey_label,
                upgrade_is_editing,
                upgrade_hotkey_is_special,
                on_position_click: on_hotkey_upgrade_position_click,
                on_hotkey_activate: on_upgrade_hotkey_activate,
            }
            AbilityTier {
                object_id,
                active_tier_index,
                total_tier_count,
                tier_label_text,
            }
        }
    }
}

assert_component!(HotkeyOverrideCard);
