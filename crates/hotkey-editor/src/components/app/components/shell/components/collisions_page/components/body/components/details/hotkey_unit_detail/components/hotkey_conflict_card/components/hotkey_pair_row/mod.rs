mod props;

use super::conflict_hotkey_badge::ConflictHotkeyBadge;
use super::conflict_hotkey_key::ConflictHotkeyKey;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_ability::ConflictAbility;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_ability_row::ConflictAbilityRow;
use dioxus::prelude::*;
pub use props::{AbilityPair, HotkeyPairRowProps};

/// The two abilities flanking the shared-key badge; renders nothing unless the
/// clash is an exact pair.
#[component]
pub fn HotkeyPairRow(props: HotkeyPairRowProps) -> Element {
    let Some(pair) = props.pair else {
        return rsx! {};
    };
    let hotkey_label = props.hotkey_label;
    let left = pair.left;
    let right = pair.right;
    rsx! {
        ConflictAbilityRow {
            ConflictAbility { ..left }
            ConflictHotkeyBadge {
                ConflictHotkeyKey { text: hotkey_label }
            }
            ConflictAbility { ..right }
        }
    }
}
