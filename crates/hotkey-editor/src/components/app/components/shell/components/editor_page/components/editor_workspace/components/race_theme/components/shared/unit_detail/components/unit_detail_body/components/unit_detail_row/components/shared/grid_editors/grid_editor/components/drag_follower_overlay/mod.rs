pub mod components;
mod props;

use components::drag_follower_ghost::{DragFollowerGhost, DragFollowerGhostProps};
use dioxus::prelude::*;
pub use props::DragFollowerOverlayProps;
use tw_macro::assert_component;

#[component]
pub fn DragFollowerOverlay(props: DragFollowerOverlayProps) -> Element {
    rsx! {
        DragFollowerGhost { ..DragFollowerGhostProps::from(&props) }
    }
}

assert_component!(DragFollowerOverlay);
