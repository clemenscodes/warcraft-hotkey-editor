pub mod components;
mod model;
mod view;

pub use view::ConflictPanelView;

use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::panel_card::PanelCard;
use components::conflict_panel_body::ConflictPanelBodyView;
use dioxus::prelude::*;
use model::ConflictPanelModel;
use tw_macro::assert_component;

/// The conflict card: the role caption over exactly one of the two clash layouts (the pair
/// row or the multi stack). Composes the shared `PanelCard` surface, supplying its body
/// region. Shared by the hotkey and unit-position conflict cards.
#[component]
pub fn ConflictPanel(props: ConflictPanelModel) -> Element {
    let model = props.model;
    let models = vec![model];
    let body = ConflictPanelBodyView { models };
    rsx! {
        PanelCard::<ConflictPanelBodyView> { body }
    }
}

assert_component!(ConflictPanel);
