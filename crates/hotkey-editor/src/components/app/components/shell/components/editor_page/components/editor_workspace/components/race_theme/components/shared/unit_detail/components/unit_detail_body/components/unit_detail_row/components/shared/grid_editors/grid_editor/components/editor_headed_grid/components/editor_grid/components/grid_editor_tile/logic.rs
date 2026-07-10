use super::props::GridEditorTileProps;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::grid_editor::components::shared::hotkey_badge::HotkeyBadgeState;
use crate::components::app::components::shell::components::shared::grid_tile::GridTileState;
use crate::components::app::components::shell::components::shared::tile_face::TileFaceProps;
use dioxus::prelude::*;
use warcraft_keybinds::{GridCoordinate, HotkeyToken, RenderedTile};

/// A finished interactive editor tile as domain-shaped data: the paint fields plus the
/// editor's interaction — focus, drag flags, and the pointer/keyboard handlers. The
/// `GridEditor` builds these in its own reactive scope and threads them down through
/// `EditorHeadedGrid` and `EditorGrid`, which render each `GridEditorTile` from these
/// named fields. It is the tile's view-model, never another component's props.
#[derive(Clone, PartialEq)]
pub(crate) struct EditorTile {
    pub(crate) coordinate: GridCoordinate,
    pub(crate) icon: Option<String>,
    pub(crate) label: String,
    pub(crate) hotkey: HotkeyToken,
    pub(crate) badge_state: HotkeyBadgeState,
    pub(crate) state: GridTileState,
    pub(crate) is_dragging_source: bool,
    pub(crate) is_drag_over: bool,
    pub(crate) is_focusable: bool,
    pub(crate) draggable: bool,
    pub(crate) onkeydown: EventHandler<KeyboardEvent>,
    pub(crate) onpointerdown: EventHandler<PointerEvent>,
    pub(crate) onpointermove: EventHandler<PointerEvent>,
    pub(crate) onpointerup: EventHandler<PointerEvent>,
    pub(crate) onpointercancel: EventHandler<PointerEvent>,
    pub(crate) onlostpointercapture: EventHandler<PointerEvent>,
    pub(crate) onclick: EventHandler<MouseEvent>,
    pub(crate) ondoubleclick: EventHandler<MouseEvent>,
}

impl From<&RenderedTile> for EditorTile {
    /// The read-only base from a domain tile: the paint comes from `TileFaceProps`, and
    /// the editor overlays behavior on top. Every handler starts at its default (the
    /// `GridEditor` wires them); only the two interaction flags the domain decides —
    /// focusability and draggability — are read from the rendered tile.
    fn from(rendered: &RenderedTile) -> Self {
        let face = TileFaceProps::from(rendered);
        let TileFaceProps {
            coordinate,
            icon,
            label,
            hotkey,
            badge_state,
            state,
            ..
        } = face;
        let is_focusable = rendered.occupant().is_some();
        let draggable = rendered.draggable();
        Self {
            coordinate,
            icon,
            label,
            hotkey,
            badge_state,
            state,
            is_dragging_source: false,
            is_drag_over: false,
            is_focusable,
            draggable,
            onkeydown: EventHandler::default(),
            onpointerdown: EventHandler::default(),
            onpointermove: EventHandler::default(),
            onpointerup: EventHandler::default(),
            onpointercancel: EventHandler::default(),
            onlostpointercapture: EventHandler::default(),
            onclick: EventHandler::default(),
            ondoubleclick: EventHandler::default(),
        }
    }
}

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
