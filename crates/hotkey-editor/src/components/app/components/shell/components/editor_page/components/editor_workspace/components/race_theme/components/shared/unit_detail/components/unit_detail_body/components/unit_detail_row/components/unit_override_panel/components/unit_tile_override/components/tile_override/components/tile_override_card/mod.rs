pub mod components;
mod props;
mod view;

pub use view::TileOverrideCardView;
mod style;

use dioxus::prelude::*;

use components::ability_description::AbilityDescription;
use components::alt_state_section::AltStateSection;
use components::tile_override_header::TileOverrideHeader;
use components::upgrade_section::UpgradeSection;
use components::upgrade_tier::UpgradeTier;
use style::CLASS;
use tw_macro::assert_component;

use props::TileOverrideCardProps;

/// The gold-edged card holding the override panel's header and ability sections.
#[component]
pub fn TileOverrideCard(props: TileOverrideCardProps) -> Element {
    let TileOverrideCardProps {
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
    } = props;
    rsx! {
        div { class: CLASS,
            TileOverrideHeader {
                name_text, object_id, show_hotkey_field, hotkey_label, hotkey_is_editing,
                hotkey_is_special, on_hotkey_activate, show_research_field, research_label,
                research_is_editing, research_is_special, on_research_activate, is_info_only,
            }
            AbilityDescription { description_lines }
            AltStateSection {
                alt_name_text, alt_description_lines, show_alt_controls, alt_hotkey_label,
                alt_hotkey_is_editing, alt_hotkey_is_special_token,
                on_position_click: on_alt_position_click,
                on_hotkey_activate: on_alt_hotkey_activate,
            }
            UpgradeSection {
                show: upgrade_show, upgrade_hotkey_label, upgrade_is_editing,
                upgrade_hotkey_is_special,
                on_position_click: on_upgrade_position_click,
                on_hotkey_activate: on_upgrade_hotkey_activate,
            }
            UpgradeTier { object_id, active_tier_index, total_tier_count, tier_label_text }
        }
    }
}

assert_component!(TileOverrideCard);
