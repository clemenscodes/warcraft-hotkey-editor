mod components;
mod data;
mod logic;
mod props;

use super::detail::{Detail, DetailBody, DetailContent};
use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::collision_count::CollisionCount;
use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::coordinate::Coordinate;
use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::mini_grid::{
    MiniGrid, MiniGridProps,
};
use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::row_meta::RowMeta;
use crate::components::app::components::shell::components::collisions_page::logic::CarrierDialogData;
use components::carriers_dialog_host::CarriersDialogHost;
use components::island_conflict_card::IslandConflictCard;
use dioxus::prelude::*;
use logic::selected;
pub use props::IslandDetailProps;

/// The island (position-collision) detail extension: builds the mini-grid header and
/// per-unit conflict cards, fed into the base detail pane. The carriers dialog opens
/// over the pane when an ability is clicked.
#[component]
pub fn IslandDetail(props: IslandDetailProps) -> Element {
    let carrier_dialog = use_signal(|| None::<CarrierDialogData>);
    let view_navigation = props.view_navigation;
    let Some(model) = selected(&props, carrier_dialog) else {
        let content = DetailContent::Empty(data::EMPTY_PROMPT);
        return rsx! {
            Detail { content }
        };
    };
    let coordinate = model.coordinate;
    let mini_grid = MiniGridProps { coordinate };
    let header = rsx! {
        MiniGrid { ..mini_grid }
        RowMeta {
            Coordinate { coordinate }
            CollisionCount { count: model.count }
        }
    };
    let cards = rsx! {
        for card in model.cards {
            IslandConflictCard { ..card }
        }
    };
    let body = DetailBody { header, cards };
    let content = DetailContent::Loaded(body);
    rsx! {
        Detail { content }
        CarriersDialogHost { carrier_dialog, view_navigation }
    }
}
