use dioxus::prelude::*;
use warcraft_api::RaceLabels;

use super::props::GridTileProps;

/// Everything the tile's outer `div` needs: the class list and attributes derived
/// from the state flags, the grid coordinate as DOM attributes, and the forwarded
/// event handlers. Field names match the attributes they feed, so the markup uses
/// RSX shorthand. The component file builds none of this; it destructures this and
/// renders.
pub(super) struct GridTilePresentation {
    pub(super) class: String,
    pub(super) tabindex: &'static str,
    pub(super) draggable_attribute: &'static str,
    pub(super) race_attribute: &'static str,
    pub(super) row: u8,
    pub(super) column: u8,
    pub(super) onkeydown: EventHandler<KeyboardEvent>,
    pub(super) onpointerdown: EventHandler<PointerEvent>,
    pub(super) onpointermove: EventHandler<PointerEvent>,
    pub(super) onpointerup: EventHandler<PointerEvent>,
    pub(super) onpointercancel: EventHandler<PointerEvent>,
    pub(super) onlostpointercapture: EventHandler<PointerEvent>,
    pub(super) onclick: EventHandler<MouseEvent>,
    pub(super) ondoubleclick: EventHandler<MouseEvent>,
}

impl From<&GridTileProps> for GridTilePresentation {
    /// Derives the tile's class list, focus, draggable, race, and coordinate
    /// attributes from its state flags and copies its event handlers.
    fn from(props: &GridTileProps) -> Self {
        let mut class = String::from("grid-tile");
        let base = props.state.base_class();
        if !base.is_empty() {
            class.push(' ');
            class.push_str(base);
        }
        if props.is_dragging_source {
            class.push_str(" dragging-source");
        }
        if props.is_drag_over {
            class.push_str(" drag-over");
        }
        let tabindex = if props.is_focusable { "0" } else { "-1" };
        let draggable_attribute = if props.draggable { "true" } else { "false" };
        let race_attribute = RaceLabels::data_attribute(props.race);
        let column_index = props.coordinate.column();
        let row_index = props.coordinate.row();
        let column = u8::from(column_index);
        let row = u8::from(row_index);
        let onkeydown = props.onkeydown;
        let onpointerdown = props.onpointerdown;
        let onpointermove = props.onpointermove;
        let onpointerup = props.onpointerup;
        let onpointercancel = props.onpointercancel;
        let onlostpointercapture = props.onlostpointercapture;
        let onclick = props.onclick;
        let ondoubleclick = props.ondoubleclick;
        Self {
            class,
            tabindex,
            draggable_attribute,
            race_attribute,
            row,
            column,
            onkeydown,
            onpointerdown,
            onpointermove,
            onpointerup,
            onpointercancel,
            onlostpointercapture,
            onclick,
            ondoubleclick,
        }
    }
}
