pub mod components;
mod logic;
mod props;
mod style;

use super::conflict_grid::ConflictGrid;
use super::conflict_object_id::ConflictObjectId;
use super::hotkey_detail_unit::HotkeyDetailUnit;
use super::island_collision_count::IslandCollisionCount;
use super::island_coord::IslandCoord;
use super::island_detail_header::IslandDetailHeader;
use super::island_row_meta::IslandRowMeta;
use components::unit_position_conflict_card::UnitPositionConflictCard;
use crate::assert_component;
use dioxus::prelude::*;
use logic::selected;
pub use props::UnitPositionDetailProps;
use style::CLASS;
assert_component!(UnitPositionDetail);

/// Per-unit position-collision detail pane: a header naming the selected unit and
/// one card per command-card cell where its own abilities collide. Empty until a
/// unit is selected.
#[component]
pub fn UnitPositionDetail(props: UnitPositionDetailProps) -> Element {
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
                    UnitPositionConflictCard { ..card }
                }
            }
        }
    }
}
