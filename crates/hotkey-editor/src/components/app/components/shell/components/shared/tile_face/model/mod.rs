use super::view::TileFaceView;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::shared::grid_editor::components::shared::hotkey_badge::HotkeyBadgeState;
use crate::components::app::components::shell::components::shared::grid_tile::GridTileState;
use dioxus::prelude::*;
use warcraft_keybinds::{ColumnIndex, GridCoordinate, HotkeyToken, RowIndex};

#[derive(Props, Clone, PartialEq)]
pub struct TileFaceModel {
    #[props(default = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Zero))]
    pub coordinate: GridCoordinate,
    #[props(default)]
    pub icon: Option<String>,
    #[props(default)]
    pub label: String,
    pub hotkey: HotkeyToken,
    #[props(default)]
    pub badge_state: HotkeyBadgeState,
    #[props(default)]
    pub state: GridTileState,
    #[props(default)]
    pub is_dragging_source: bool,
    #[props(default)]
    pub is_drag_over: bool,
}

impl From<&TileFaceView> for TileFaceModel {
    fn from(view: &TileFaceView) -> Self {
        let coordinate = view.coordinate;
        let icon = view.icon.clone();
        let label = view.label.clone();
        let hotkey = view.hotkey;
        let badge_state = view.badge_state;
        let state = view.state;
        let is_dragging_source = view.is_dragging_source;
        let is_drag_over = view.is_drag_over;
        Self {
            coordinate,
            icon,
            label,
            hotkey,
            badge_state,
            state,
            is_dragging_source,
            is_drag_over,
        }
    }
}

impl ddd::Model for TileFaceModel {
    type View = TileFaceView;
}
