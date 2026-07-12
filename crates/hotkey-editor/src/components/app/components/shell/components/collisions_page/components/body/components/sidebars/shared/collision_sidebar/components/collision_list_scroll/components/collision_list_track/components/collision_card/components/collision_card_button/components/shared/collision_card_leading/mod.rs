pub mod components;
mod model;
mod view;

pub use view::CollisionCardLeadingView;

use crate::components::app::components::shell::components::collisions_page::components::body::components::sidebars::shared::collision_sidebar::components::collision_list_scroll::components::collision_list_track::components::collision_card::state::CollisionCardContent;
use components::hotkey_unit_row_icon::HotkeyUnitRowIcon;
use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::mini_grid::MiniGrid;
use dioxus::prelude::*;
use tw_macro::assert_component;
use model::CollisionCardLeadingModel;

/// The leading visual of a collision card: the unit's portrait for a unit card, or
/// the highlighted mini grid for an island card. A dedicated switch on the card
/// content, so the card body stays a flat list of children.
#[component]
pub fn CollisionCardLeading(props: CollisionCardLeadingModel) -> Element {
    match props.content {
        CollisionCardContent::Unit { icon_url, name, .. } => {
            rsx! {
                HotkeyUnitRowIcon { icon_url, alt: name }
            }
        }
        CollisionCardContent::Island { coordinate } => {
            rsx! {
                MiniGrid { coordinate }
            }
        }
    }
}

assert_component!(CollisionCardLeading);
