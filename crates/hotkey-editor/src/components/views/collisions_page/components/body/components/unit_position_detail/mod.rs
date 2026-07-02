pub mod components;
mod data;
mod logic;
mod props;
mod style;

use super::collision_count::CollisionCount;
use super::conflict_grid::ConflictGrid;
use super::conflict_object_id::ConflictObjectId;
use super::coordinate::Coordinate;
use super::detail_header::DetailHeader;
use super::hotkey_detail_unit::HotkeyDetailUnit;
use super::row_meta::RowMeta;
use crate::assert_component;
use components::unit_position_conflict_card::UnitPositionConflictCard;
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
                p { {data::EMPTY_PROMPT} }
            }
        };
    };
    rsx! {
        section {
            class: CLASS,
            DetailHeader {
                HotkeyDetailUnit { ..model.unit }
                RowMeta {
                    Coordinate { text: model.name }
                    ConflictObjectId { text: model.unit_id_label }
                    CollisionCount { text: model.count_text }
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
