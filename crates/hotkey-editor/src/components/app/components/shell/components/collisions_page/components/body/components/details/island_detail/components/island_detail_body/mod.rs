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

/// The island detail card's body region. A dispatcher: when an island is selected it renders
/// the filled pane (its mini-grid coordinate header over the conflict cards), otherwise the
/// empty prompt. The selection is read from collision-selection context. It renders no
/// surface — the filled and empty panes carry their own inner layout, inside the shared
/// `DetailCard`.
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
