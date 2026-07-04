pub mod components;
mod props;

use components::drag_follower_ghost::{DragFollowerGhost, DragFollowerGhostProps};
use dioxus::prelude::*;
pub use props::DragFollowerOverlayProps;

#[component]
pub fn DragFollowerOverlay(props: DragFollowerOverlayProps) -> Element {
    rsx! {
        DragFollowerGhost { ..DragFollowerGhostProps::from(&props) }
    }
}
