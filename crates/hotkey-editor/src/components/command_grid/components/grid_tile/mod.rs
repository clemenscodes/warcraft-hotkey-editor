mod components;
mod props;
mod state;
mod style;

use dioxus::prelude::*;
use warcraft_api::RaceLabels;

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

    let mut class_name = String::from("grid-tile");
    let base = state.base_class();
    if !base.is_empty() {
        class_name.push(' ');
        class_name.push_str(base);
    }
    if is_dragging_source {
        class_name.push_str(" dragging-source");
    }
    if is_drag_over {
        class_name.push_str(" drag-over");
    }

    let tabindex_value = if is_focusable { "0" } else { "-1" };
    let draggable_attr = if draggable { "true" } else { "false" };
    let race_attr = RaceLabels::data_attribute(race);

    rsx! {
        for style_sheet in GRID_TILE_STYLE_SHEETS {
            document::Stylesheet { href: style_sheet }
        }
        div {
            class: class_name,
            tabindex: tabindex_value,
            "data-race": race_attr,
            "data-draggable": draggable_attr,
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
                span { class: "command-label", {label} }
            }
            if let Some(letter_text) = hotkey {
                div { class: "grid-tile-badge",
                    HotkeyBadge { letter: letter_text, state: badge_state }
                }
            }
        }
    }
}
