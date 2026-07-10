use super::props::GridEditorTileProps;
use dioxus::prelude::*;

/// Everything the editor tile's interactive wrapper renders: the focus tabindex and
/// every forwarded event handler. Field names match the attributes they feed, so the
/// wrapper spreads them with RSX shorthand. This is the interaction that the inert base
/// `GridTile` deliberately does not carry; the drag-state looks are the painter's own
/// mounted overlays, and draggability is the `DraggableMarker`, so no look-flag
/// attributes live here.
#[derive(Clone, PartialEq)]
pub struct EditorTileChrome {
    pub(super) tabindex: &'static str,
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
