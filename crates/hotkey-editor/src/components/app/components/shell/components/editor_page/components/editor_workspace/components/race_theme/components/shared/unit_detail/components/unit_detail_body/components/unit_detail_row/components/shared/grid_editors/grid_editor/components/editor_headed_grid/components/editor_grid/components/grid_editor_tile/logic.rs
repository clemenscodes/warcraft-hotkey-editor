use super::props::GridEditorTileProps;
use dioxus::prelude::*;

/// Everything the editor tile's interactive wrapper renders: the focus and
/// draggable attributes, the drag-state markers, and every forwarded event
/// handler. Field names match the attributes they feed, so the wrapper spreads
/// them with RSX shorthand. This is the interaction that the inert base `GridTile`
/// deliberately does not carry.
#[derive(Clone, PartialEq)]
pub struct EditorTileChrome {
    pub(super) tabindex: &'static str,
    pub(super) draggable_attribute: &'static str,
    pub(super) dragging_source: &'static str,
    pub(super) drag_over: &'static str,
    pub(super) onkeydown: EventHandler<KeyboardEvent>,
    pub(super) onpointerdown: EventHandler<PointerEvent>,
    pub(super) onpointermove: EventHandler<PointerEvent>,
    pub(super) onpointerup: EventHandler<PointerEvent>,
    pub(super) onpointercancel: EventHandler<PointerEvent>,
    pub(super) onlostpointercapture: EventHandler<PointerEvent>,
    pub(super) onclick: EventHandler<MouseEvent>,
    pub(super) ondoubleclick: EventHandler<MouseEvent>,
}

impl From<&GridEditorTileProps> for EditorTileChrome {
    fn from(props: &GridEditorTileProps) -> Self {
        let tabindex = if props.is_focusable { "0" } else { "-1" };
        let draggable_attribute = if props.draggable { "true" } else { "false" };
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
