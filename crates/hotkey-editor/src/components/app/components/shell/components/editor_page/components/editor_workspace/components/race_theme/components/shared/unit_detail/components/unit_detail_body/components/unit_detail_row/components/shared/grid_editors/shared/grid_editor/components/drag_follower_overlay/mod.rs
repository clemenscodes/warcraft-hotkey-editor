pub mod components;
mod model;
mod presentation;
mod view;

pub use view::DragFollowerOverlayView;

use components::drag_follower_ghost::DragFollowerGhost;
use dioxus::prelude::*;
use model::DragFollowerOverlayModel;
use presentation::use_drag_follower_overlay;
use tw_macro::assert_component;

#[component]
pub fn DragFollowerOverlay(props: DragFollowerOverlayModel) -> Element {
    let presentation = use_drag_follower_overlay(&props);
    rsx! {
        DragFollowerGhost { presentation }
    }
}

assert_component!(DragFollowerOverlay);
