mod logic;
mod props;
mod style;

use dioxus::prelude::*;

use crate::components::command_grid::HotkeyBadge;
use logic::{FollowerPresentation, OverlayRender};
use style::DRAG_FOLLOWER_STYLES;

pub use props::DragFollowerOverlayProps;

#[component]
pub fn DragFollowerOverlay(props: DragFollowerOverlayProps) -> Element {
    let DragFollowerOverlayProps {
        drag_follower,
        race,
        visible,
    } = props;

    let OverlayRender {
        race_attribute,
        follower,
    } = OverlayRender::new(race, visible, drag_follower);

    rsx! {
        document::Stylesheet { href: DRAG_FOLLOWER_STYLES }
        if let Some(FollowerPresentation {
            class_name,
            position_style,
            badge_state,
            icon_source,
            label_text,
            letter,
        }) = follower
        {
            div { class: class_name, "data-race": race_attribute, style: position_style,
                if let Some(source) = icon_source {
                    img { src: source, alt: label_text, draggable: "false", decoding: "async" }
                } else {
                    span { class: "drag-follower-label", {label_text} }
                }
                if let Some(letter_text) = letter {
                    div { class: "drag-follower-badge",
                        HotkeyBadge { letter: letter_text, state: badge_state }
                    }
                }
            }
        }
    }
}
