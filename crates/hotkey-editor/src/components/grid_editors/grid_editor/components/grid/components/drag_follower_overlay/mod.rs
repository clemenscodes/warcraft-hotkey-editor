mod components;
mod props;
mod style;

use dioxus::prelude::*;

use components::{DragFollowerGhost, DragFollowerGhostProps};
use style::DRAG_FOLLOWER_STYLES;

pub use props::DragFollowerOverlayProps;

#[component]
pub fn DragFollowerOverlay(props: DragFollowerOverlayProps) -> Element {
    rsx! {
        document::Stylesheet { href: DRAG_FOLLOWER_STYLES }
        DragFollowerGhost { ..DragFollowerGhostProps::from(&props) }
    }
}
