mod logic;
mod props;

use crate::components::views::collisions_page::conflict_ability::ConflictAbility;
use crate::components::views::collisions_page::conflict_ability_row::ConflictAbilityRow;
use crate::components::views::collisions_page::conflict_card::ConflictCard;
use crate::components::views::collisions_page::conflict_card_caption::ConflictCardCaption;
use crate::components::views::collisions_page::conflict_hotkey_badge::ConflictHotkeyBadge;
use crate::components::views::collisions_page::conflict_hotkey_key::ConflictHotkeyKey;
use dioxus::prelude::*;
use logic::{AbilityPair, HotkeyConflictCardModel};
pub use props::HotkeyConflictCardProps;

/// One shared-hotkey conflict card: the abilities flanking (or stacked under) the
/// shared key badge, captioned by the command card the clash lives on.
#[component]
pub fn HotkeyConflictCard(props: HotkeyConflictCardProps) -> Element {
    let HotkeyConflictCardModel {
        hotkey_label,
        role_label,
        pair,
        multi,
    } = HotkeyConflictCardModel::from(&props);
    rsx! {
        ConflictCard {
            ConflictCardCaption { text: role_label }
            if let Some(AbilityPair { left, right }) = pair {
                ConflictAbilityRow {
                    ConflictAbility { ..left }
                    ConflictHotkeyBadge {
                        ConflictHotkeyKey { text: hotkey_label }
                    }
                    ConflictAbility { ..right }
                }
            } else {
                ConflictHotkeyBadge {
                    is_top: true,
                    ConflictHotkeyKey { text: hotkey_label }
                }
                ConflictAbilityRow {
                    is_multi: true,
                    for ability in multi {
                        ConflictAbility { ..ability }
                    }
                }
            }
        }
    }
}
