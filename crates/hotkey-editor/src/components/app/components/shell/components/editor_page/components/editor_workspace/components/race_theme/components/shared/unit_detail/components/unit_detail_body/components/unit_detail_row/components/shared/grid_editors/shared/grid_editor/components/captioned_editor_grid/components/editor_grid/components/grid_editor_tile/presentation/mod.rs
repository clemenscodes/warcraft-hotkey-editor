use super::model::GridEditorTileModel;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::shared::grid_editor::components::shared::hotkey_badge::HotkeyBadgeState;
use crate::components::app::components::shell::components::shared::grid_tile::GridTileState;
use crate::components::app::components::shell::components::shared::tile_face::TileFaceView;
use dioxus::prelude::*;
use warcraft_keybinds::{GridCoordinate, HotkeyToken, RenderedTile};

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
    fn from(rendered: &RenderedTile) -> Self {
        let face = TileFaceView::from(rendered);
        let TileFaceView {
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

impl From<&GridEditorTileModel> for EditorTileChrome {
    fn from(props: &GridEditorTileModel) -> Self {
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

impl ddd::Presentation for EditorTileChrome {
    type Model = GridEditorTileModel;
}
