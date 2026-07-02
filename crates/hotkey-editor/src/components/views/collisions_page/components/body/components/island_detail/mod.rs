pub mod components;
mod data;
mod logic;
mod props;
mod style;

use super::collision_count::CollisionCount;
use super::conflict_grid::ConflictGrid;
use super::coordinate::Coordinate;
use super::coordinate_group::CoordinateGroup;
use super::coordinate_separator::CoordinateSeparator;
use super::detail_header::DetailHeader;
use super::mini_grid::{MiniGrid, MiniGridProps};
use super::row_meta::RowMeta;
use crate::assert_component;
use crate::components::views::collisions_page::logic::CarrierDialogData;
use components::carriers_dialog_host::CarriersDialogHost;
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
                p { {data::EMPTY_PROMPT} }
            }
        };
    };
    let mini_grid = MiniGridProps {
        collision_column: model.position_column,
        collision_row: model.position_row,
    };
    rsx! {
        section {
            class: CLASS,
            DetailHeader {
                MiniGrid { ..mini_grid }
                RowMeta {
                    CoordinateGroup {
                        Coordinate { text: model.column_label }
                        CoordinateSeparator {}
                        Coordinate { text: model.row_label }
                    }
                    CollisionCount { text: model.count_text }
                }
            }
            ConflictGrid {
                for card in model.cards {
                    IslandConflictCard { ..card }
                }
            }
        }
        CarriersDialogHost { carrier_dialog, view_navigation }
    }
}
