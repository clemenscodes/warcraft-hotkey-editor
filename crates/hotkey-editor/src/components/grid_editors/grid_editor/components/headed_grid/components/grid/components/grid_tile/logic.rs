use dioxus::prelude::*;
use warcraft_api::RaceLabels;

use super::props::GridTileProps;

/// The chrome both the filled and empty tiles render: the focus, draggable, race,
/// and coordinate attributes, the drag-state markers, and every forwarded event
/// handler. Field names match the attributes they feed, so each tile spreads them
/// with RSX shorthand. The dispatcher builds this once from the slot's props and
/// hands it to whichever tile it renders.
#[derive(Clone, PartialEq)]
pub struct TileChrome {
    pub tabindex: &'static str,
    pub draggable_attribute: &'static str,
    pub race_attribute: &'static str,
    pub row: u8,
    pub column: u8,
    pub dragging_source: &'static str,
    pub drag_over: &'static str,
    pub onkeydown: EventHandler<KeyboardEvent>,
    pub onpointerdown: EventHandler<PointerEvent>,
    pub onpointermove: EventHandler<PointerEvent>,
    pub onpointerup: EventHandler<PointerEvent>,
    pub onpointercancel: EventHandler<PointerEvent>,
    pub onlostpointercapture: EventHandler<PointerEvent>,
    pub onclick: EventHandler<MouseEvent>,
    pub ondoubleclick: EventHandler<MouseEvent>,
}

impl From<&GridTileProps> for TileChrome {
    /// Derives the focus, draggable, race, coordinate, and drag-marker attributes
    /// from the slot's state flags and copies its event handlers.
    fn from(props: &GridTileProps) -> Self {
        let tabindex = if props.is_focusable { "0" } else { "-1" };
        let draggable_attribute = if props.draggable { "true" } else { "false" };
        let race_attribute = RaceLabels::data_attribute(props.race);
        let column_index = props.coordinate.column();
        let row_index = props.coordinate.row();
        let column = u8::from(column_index);
        let row = u8::from(row_index);
        let dragging_source = if props.is_dragging_source {
            "true"
        } else {
            "false"
        };
        let drag_over = if props.is_drag_over { "true" } else { "false" };
        let onkeydown = props.onkeydown;
        let onpointerdown = props.onpointerdown;
        let onpointermove = props.onpointermove;
        let onpointerup = props.onpointerup;
        let onpointercancel = props.onpointercancel;
        let onlostpointercapture = props.onlostpointercapture;
        let onclick = props.onclick;
        let ondoubleclick = props.ondoubleclick;
        Self {
            tabindex,
            draggable_attribute,
            race_attribute,
            row,
            column,
            dragging_source,
            drag_over,
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
