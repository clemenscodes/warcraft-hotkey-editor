pub mod components;
mod hooks;
mod props;

use components::drag_follower_ghost::DragFollowerGhost;
use dioxus::prelude::*;
use hooks::use_drag_follower_overlay;
use props::DragFollowerOverlayProps;
use tw_macro::assert_component;

#[component]
pub fn DragFollowerOverlay(props: DragFollowerOverlayProps) -> Element {
    let presentation = use_drag_follower_overlay(&props);
    rsx! {
        DragFollowerGhost { presentation }
    }
}

assert_component!(DragFollowerOverlay);
