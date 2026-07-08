mod components;
mod data;
mod logic;
mod props;

use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_detail_unit::ConflictDetailUnit;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_grid::ConflictGrid;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::detail::Detail;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::detail_header::DetailHeader;
use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::collision_count::CollisionCount;
use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::conflict_object_id::ConflictObjectId;
use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::conflict_unit_name::ConflictUnitName;
use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::row_meta::RowMeta;
use components::unit_position_conflict_card::UnitPositionConflictCard;
use dioxus::prelude::*;
use logic::UnitPositionDetailData;
pub use props::UnitPositionDetailProps;
use tw_macro::assert_component;

assert_component!(UnitPositionDetail);

/// The per-unit position-collision detail extension: resolves the selected unit into
/// typed detail data and fills the base detail pane with its header and
/// position-conflict cards.
#[component]
pub fn UnitPositionDetail(props: UnitPositionDetailProps) -> Element {
    let Some(data) = logic::selected(&props) else {
        return rsx! {
            Detail {
                is_empty: true,
                p { {data::EMPTY_PROMPT} }
            }
        };
    };
    let UnitPositionDetailData {
        unit,
        name,
        unit_id_label,
        count,
        cards,
    } = data;
    rsx! {
        Detail {
            DetailHeader {
                ConflictDetailUnit { ..unit }
                RowMeta {
                    ConflictUnitName { text: name }
                    ConflictObjectId { text: unit_id_label }
                    CollisionCount { count }
                }
            }
            ConflictGrid {
                for card in cards {
                    UnitPositionConflictCard { ..card }
                }
            }
        }
    }
}
