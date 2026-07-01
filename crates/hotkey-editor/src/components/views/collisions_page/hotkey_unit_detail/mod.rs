pub mod components;
mod logic;
mod props;
mod style;

use super::conflict_grid::ConflictGrid;
use super::conflict_object_id::ConflictObjectId;
use super::island_collision_count::IslandCollisionCount;
use super::island_coord::IslandCoord;
use super::island_detail_header::IslandDetailHeader;
use super::island_row_meta::IslandRowMeta;
use components::hotkey_conflict_card::HotkeyConflictCard;
use super::hotkey_detail_unit::HotkeyDetailUnit;
use crate::assert_component;
use dioxus::prelude::*;
use logic::selected;
pub use props::HotkeyUnitDetailProps;
use style::CLASS;
assert_component!(HotkeyUnitDetail);

/// The hotkey-collision detail pane: a header naming the selected unit and one card
/// per shared-letter conflict on its command cards. Empty until a unit is selected.
#[component]
pub fn HotkeyUnitDetail(props: HotkeyUnitDetailProps) -> Element {
    let Some(model) = selected(&props) else {
        return rsx! {
            section {
                class: CLASS,
                "data-empty": true,
                p { "Select a unit to inspect." }
            }
        };
    };
    rsx! {
        section {
            class: CLASS,
            IslandDetailHeader {
                HotkeyDetailUnit { ..model.unit }
                IslandRowMeta {
                    IslandCoord { text: model.name }
                    ConflictObjectId { text: model.unit_id_label }
                    IslandCollisionCount { text: model.count_text }
                }
            }
            ConflictGrid {
                for card in model.cards {
                    HotkeyConflictCard { ..card }
                }
            }
        }
    }
}
