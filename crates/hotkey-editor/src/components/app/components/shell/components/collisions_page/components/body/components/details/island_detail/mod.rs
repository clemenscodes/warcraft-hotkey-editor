pub mod components;
mod data;
mod logic;
mod props;
mod view;

pub use view::IslandDetailView;

use crate::services::collision_selection::context::use_collision_selection;
use components::empty_island_detail::EmptyIslandDetail;
use components::filled_island_detail::FilledIslandDetail;
use dioxus::prelude::*;
use props::IslandDetailProps;
use tw_macro::assert_component;

/// The position-island detail pane. A dispatcher: when an island is selected it renders
/// the filled pane (its mini-grid coordinate header over the conflict cards), otherwise
/// the empty prompt. The selection is read from collision-selection context.
#[component]
pub fn IslandDetail(props: IslandDetailProps) -> Element {
    let selected_island = use_collision_selection().selected_island();
    if let Some(island) = logic::selected(&props, selected_island) {
        rsx! {
            FilledIslandDetail { island }
        }
    } else {
        rsx! {
            EmptyIslandDetail {}
        }
    }
}

assert_component!(IslandDetail);
