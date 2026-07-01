pub mod components;
mod props;

use dioxus::prelude::*;

use components::drag_follower_ghost::{DragFollowerGhost, DragFollowerGhostProps};

pub use props::DragFollowerOverlayProps;

#[component]
pub fn DragFollowerOverlay(props: DragFollowerOverlayProps) -> Element {
    rsx! {
        DragFollowerGhost { ..DragFollowerGhostProps::from(&props) }
    }
}
