pub mod components;
mod props;

use crate::components::app::components::shell::components::collisions_page::components::body::components::sidebars::shared::collision_sidebar::components::collision_list_scroll::components::collision_list_track::components::collision_card::state::CollisionCardContent;
use components::hotkey_unit_row_icon::{HotkeyUnitRowIcon, HotkeyUnitRowIconProps};
use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::mini_grid::{MiniGrid, MiniGridProps};
use dioxus::prelude::*;
use tw_macro::assert_component;
pub use props::CollisionCardVisualProps;

/// The leading visual of a collision card: the unit's portrait for a unit card, or
/// the highlighted mini grid for an island card. A dedicated switch on the card
/// content, so the card body stays a flat list of children.
#[component]
pub fn CollisionCardVisual(props: CollisionCardVisualProps) -> Element {
    match props.content {
        CollisionCardContent::Unit { icon_url, name, .. } => {
            let icon = HotkeyUnitRowIconProps {
                icon_url,
                alt: name,
            };
            rsx! {
                HotkeyUnitRowIcon { ..icon }
            }
        }
        CollisionCardContent::Island { coordinate } => {
            let grid = MiniGridProps { coordinate };
            rsx! {
                MiniGrid { ..grid }
            }
        }
    }
}

assert_component!(CollisionCardVisual);
