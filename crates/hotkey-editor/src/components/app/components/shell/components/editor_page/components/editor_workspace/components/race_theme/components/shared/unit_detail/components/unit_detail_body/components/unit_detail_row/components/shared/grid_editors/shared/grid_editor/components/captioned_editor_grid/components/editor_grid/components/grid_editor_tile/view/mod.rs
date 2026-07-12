use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::shared::grid_editor::components::shared::hotkey_badge::HotkeyBadgeState;
use crate::components::app::components::shell::components::shared::grid_tile::GridTileState;
use dioxus::prelude::*;
use warcraft_keybinds::{GridCoordinate, HotkeyToken};

/// The published `View` contract mirroring [`GridEditorTileModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct GridEditorTileView {
    pub coordinate: GridCoordinate,
    pub icon: Option<String>,
    pub label: String,
    /// The hotkey; every editor tile always has one, shown as its badge.
    pub hotkey: HotkeyToken,
    pub badge_state: HotkeyBadgeState,
    pub state: GridTileState,
    pub is_dragging_source: bool,
    pub is_drag_over: bool,
    pub is_focusable: bool,
    pub draggable: bool,
    pub onkeydown: EventHandler<KeyboardEvent>,
    pub onpointerdown: EventHandler<PointerEvent>,
    pub onpointermove: EventHandler<PointerEvent>,
    pub onpointerup: EventHandler<PointerEvent>,
    pub onpointercancel: EventHandler<PointerEvent>,
    pub onlostpointercapture: EventHandler<PointerEvent>,
    pub onclick: EventHandler<MouseEvent>,
    pub ondoubleclick: EventHandler<MouseEvent>,
}

impl ddd::View for GridEditorTileView {}
