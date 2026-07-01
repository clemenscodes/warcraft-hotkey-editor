pub mod components;
mod logic;
mod props;
mod style;

use super::CarrierDialogData;
use super::conflict_grid::ConflictGrid;
use super::island_collision_count::IslandCollisionCount;
use super::island_coord::IslandCoord;
use super::island_coord_group::IslandCoordGroup;
use super::island_coord_sep::IslandCoordSep;
use super::island_detail_header::IslandDetailHeader;
use super::island_mini_grid::{IslandMiniGrid, IslandMiniGridProps};
use super::island_row_meta::IslandRowMeta;
use crate::assert_component;
use components::carriers_dialog::CarriersDialog;
use components::island_conflict_card::IslandConflictCard;
use dioxus::prelude::*;
use logic::selected;
pub use props::IslandDetailProps;
use style::CLASS;
assert_component!(IslandDetail);

/// Island detail pane: a header mirroring the island card (mini grid, coordinate,
/// collision count), then one card per affected unit. Empty until a collision is
/// selected. The carriers dialog opens over the pane when an ability is clicked.
#[component]
pub fn IslandDetail(props: IslandDetailProps) -> Element {
    let carrier_dialog = use_signal(|| None::<CarrierDialogData>);
    let view_navigation = props.view_navigation;
    let Some(model) = selected(&props, carrier_dialog) else {
        return rsx! {
            section {
                class: CLASS,
                "data-empty": true,
                p { "Select a collision to inspect." }
            }
        };
    };
    let mini_grid = IslandMiniGridProps {
        collision_column: model.position_column,
        collision_row: model.position_row,
    };
    let dialog_state = carrier_dialog.read().clone();
    rsx! {
        section {
            class: CLASS,
            IslandDetailHeader {
                IslandMiniGrid { ..mini_grid }
                IslandRowMeta {
                    IslandCoordGroup {
                        IslandCoord { text: model.column_label }
                        IslandCoordSep {}
                        IslandCoord { text: model.row_label }
                    }
                    IslandCollisionCount { text: model.count_text }
                }
            }
            ConflictGrid {
                for card in model.cards {
                    IslandConflictCard { ..card }
                }
            }
        }
        if let Some(dialog_data) = dialog_state {
            CarriersDialog { dialog_data, carrier_dialog, view_navigation }
        }
    }
}
