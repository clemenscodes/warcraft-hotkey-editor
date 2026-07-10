use super::view::GridEditorTileView;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::grid_editor::components::shared::hotkey_badge::HotkeyBadgeState;
use crate::components::app::components::shell::components::shared::grid_tile::GridTileState;
use dioxus::prelude::*;
use warcraft_keybinds::{ColumnIndex, GridCoordinate, HotkeyToken, RowIndex};

/// The interactive editor tile's props: the `TileFace` painter's visual fields plus the
/// editor's interaction — focus, drag state, and the event handlers. This is the only
/// tile that expects a hotkey. Its address is the domain `GridCoordinate`; its hotkey is
/// the domain `HotkeyToken`.
#[derive(Props, Clone, PartialEq)]
pub struct GridEditorTileProps {
    #[props(default = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Zero))]
    pub coordinate: GridCoordinate,
    #[props(default)]
    pub icon: Option<String>,
    #[props(default)]
    pub label: String,
    /// The hotkey; every editor tile always has one, shown as its badge.
    pub hotkey: HotkeyToken,
    #[props(default)]
    pub badge_state: HotkeyBadgeState,
    #[props(default)]
    pub state: GridTileState,
    #[props(default)]
    pub is_dragging_source: bool,
    #[props(default)]
    pub is_drag_over: bool,
    #[props(default)]
    pub is_focusable: bool,
    #[props(default)]
    pub draggable: bool,
    #[props(default)]
    pub onkeydown: EventHandler<KeyboardEvent>,
    #[props(default)]
    pub onpointerdown: EventHandler<PointerEvent>,
    #[props(default)]
    pub onpointermove: EventHandler<PointerEvent>,
    #[props(default)]
    pub onpointerup: EventHandler<PointerEvent>,
    #[props(default)]
    pub onpointercancel: EventHandler<PointerEvent>,
    #[props(default)]
    pub onlostpointercapture: EventHandler<PointerEvent>,
    #[props(default)]
    pub onclick: EventHandler<MouseEvent>,
    #[props(default)]
    pub ondoubleclick: EventHandler<MouseEvent>,
}

impl From<&GridEditorTileView> for GridEditorTileProps {
    fn from(view: &GridEditorTileView) -> Self {
        let GridEditorTileView {
            coordinate,
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
        } = view.clone();
        Self {
            coordinate,
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
        }
    }
}

impl ddd::Props for GridEditorTileProps {
    type View = GridEditorTileView;
}
