pub mod components;
mod model;
mod presentation;
mod view;

pub use view::IslandDetailBodyView;

use crate::services::collision_selection::context::use_collision_selection;
use components::empty_island_detail::EmptyIslandDetail;
use components::filled_island_detail::FilledIslandDetail;
use dioxus::prelude::*;
use model::IslandDetailBodyModel;
use tw_macro::assert_component;

#[component]
pub fn IslandDetailBody(props: IslandDetailBodyModel) -> Element {
    let selected_island = use_collision_selection().selected_island();
    if let Some(island) = presentation::selected(&props, selected_island) {
        rsx! {
            FilledIslandDetail {
                island,
            }
        }
    } else {
        rsx! {
            EmptyIslandDetail {
                prompt: presentation::EMPTY_PROMPT,
            }
        }
    }
}

assert_component!(IslandDetailBody);
