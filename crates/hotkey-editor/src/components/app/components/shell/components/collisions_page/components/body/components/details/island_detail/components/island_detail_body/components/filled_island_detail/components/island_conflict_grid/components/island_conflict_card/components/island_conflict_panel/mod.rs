pub mod components;
mod model;
mod view;

pub use view::IslandConflictPanelView;

use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::panel_card::PanelCard;
use components::island_conflict_panel_body::{IslandConflictCardData, IslandConflictPanelBodyView};
use dioxus::prelude::*;
use model::IslandConflictPanelModel;
use tw_macro::assert_component;

/// One island conflict card: the affected unit heading its two clashing abilities. Composes
/// the shared `PanelCard` surface, supplying its body region.
#[component]
pub fn IslandConflictPanel(props: IslandConflictPanelModel) -> Element {
    let IslandConflictPanelModel {
        unit,
        own_ability,
        shared_ability,
    } = props;
    let card = IslandConflictCardData {
        unit,
        own_ability,
        shared_ability,
    };
    let cards = vec![card];
    let body = IslandConflictPanelBodyView { cards };
    rsx! {
        PanelCard::<IslandConflictPanelBodyView> {
            body,
        }
    }
}

assert_component!(IslandConflictPanel);
