pub mod components;
mod model;
mod view;

pub use view::IslandDetailView;

use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::detail_card::DetailCard;
use components::island_detail_body::IslandDetailBodyView;
use dioxus::prelude::*;
use model::IslandDetailModel;
use tw_macro::assert_component;

/// The position-island detail pane. Composes the shared `DetailCard` surface, supplying its
/// body region — the dispatcher that shows the filled pane (the island's mini-grid
/// coordinate header over the conflict cards) or the empty prompt.
#[component]
pub fn IslandDetail(props: IslandDetailModel) -> Element {
    let islands = props.islands;
    let body = IslandDetailBodyView { islands };
    rsx! {
        DetailCard::<IslandDetailBodyView> {
            body,
        }
    }
}

assert_component!(IslandDetail);
