mod components;
mod logic;
mod props;
mod state;
mod style;

use dioxus::prelude::*;

use logic::GridTilePresentation;
use style::GRID_TILE_STYLE_SHEETS;

pub use components::{HotkeyBadge, HotkeyBadgeProps, HotkeyBadgeState};
pub use props::GridTileProps;
pub use state::GridTileState;

#[component]
pub fn GridTile(props: GridTileProps) -> Element {
    let GridTileProps {
        race,
        icon,
        label,
        hotkey,
        badge_state,
        state,
        is_dragging_source,
        is_drag_over,
        is_focusable,
        draggable,
        onkeydown,
        onpointerdown,
        onpointermove,
        onpointerup,
        onpointercancel,
        onlostpointercapture,
        onclick,
        ondoubleclick,
        attributes,
    } = props;

    let GridTilePresentation {
        class_name,
        tabindex,
        draggable_attribute,
        race_attribute,
    } = GridTilePresentation::new(
        state,
        is_dragging_source,
        is_drag_over,
        is_focusable,
        draggable,
        race,
    );

    rsx! {
        for style_sheet in GRID_TILE_STYLE_SHEETS {
            document::Stylesheet { href: style_sheet }
        }
        div {
            class: class_name,
            tabindex,
            "data-race": race_attribute,
            "data-draggable": draggable_attribute,
            onkeydown,
            onpointerdown,
            onpointermove,
            onpointerup,
            onpointercancel,
            onlostpointercapture,
            onclick,
            ondoubleclick,
            ..attributes,
            if let Some(source) = icon {
                img {
                    src: source,
                    alt: label,
                    draggable: "false",
                    loading: "lazy",
                    decoding: "async",
                }
            } else if is_focusable {
                span { class: "grid-tile-label", {label} }
            }
            if let Some(letter_text) = hotkey {
                div { class: "grid-tile-badge",
                    HotkeyBadge { letter: letter_text, state: badge_state }
                }
            }
        }
    }
}
