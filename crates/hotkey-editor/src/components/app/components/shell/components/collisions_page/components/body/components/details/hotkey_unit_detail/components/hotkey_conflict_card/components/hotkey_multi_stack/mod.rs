mod props;

use super::conflict_hotkey_badge::ConflictHotkeyBadge;
use super::conflict_hotkey_key::ConflictHotkeyKey;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_ability::ConflictAbility;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_ability_row::ConflictAbilityRow;
use dioxus::prelude::*;
pub use props::HotkeyMultiStackProps;

/// The badge stacked above every clashing ability; renders nothing when the clash
/// is an exact pair (that layout is the pair row instead).
#[component]
pub fn HotkeyMultiStack(props: HotkeyMultiStackProps) -> Element {
    let abilities = props.abilities;
    if abilities.is_empty() {
        return rsx! {};
    }
    let hotkey_label = props.hotkey_label;
    rsx! {
        ConflictHotkeyBadge {
            is_top: true,
            ConflictHotkeyKey { text: hotkey_label }
        }
        ConflictAbilityRow {
            is_multi: true,
            for ability in abilities {
                ConflictAbility { ..ability }
            }
        }
    }
}
