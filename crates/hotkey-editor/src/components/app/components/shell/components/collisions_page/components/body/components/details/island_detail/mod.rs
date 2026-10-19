pub mod components;
mod model;
mod view;

pub use view::IslandDetailView;

use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::detail_card::DetailCard;
use components::island_detail_body::IslandDetailBodyView;
use dioxus::prelude::*;
use model::IslandDetailModel;
use tw_macro::assert_component;

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
