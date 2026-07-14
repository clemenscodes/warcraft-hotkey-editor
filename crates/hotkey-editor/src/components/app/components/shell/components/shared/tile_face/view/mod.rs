use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::shared::grid_editor::components::shared::hotkey_badge::HotkeyBadgeState;
use crate::components::app::components::shell::components::shared::grid_tile::GridTileState;
use crate::components::app::components::shell::components::shared::icons::IconUrl;
use dioxus::prelude::*;
use warcraft_keybinds::{ColumnIndex, GridCoordinate, HotkeyToken, RenderedTile, RowIndex};

#[derive(Props, Clone, PartialEq)]
pub struct TileFaceView {
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

impl From<&RenderedTile> for TileFaceView {
    fn from(rendered: &RenderedTile) -> Self {
        let coordinate = rendered.coordinate();
        let icon = rendered
            .icon()
            .map(IconUrl::from_icon_path)
            .map(|url| url.to_string());
        let label = rendered.display_name().to_string();
        let hotkey = rendered.hotkey();
        let badge_state = if rendered.is_conflict() {
            HotkeyBadgeState::Conflict
        } else if rendered.is_passive() {
            HotkeyBadgeState::Passive
        } else {
            HotkeyBadgeState::Normal
        };
        let state = if rendered.occupant().is_none() {
            GridTileState::Empty
        } else if rendered.is_selected() {
            GridTileState::Selected
        } else if rendered.is_command() {
            GridTileState::Command
        } else {
            GridTileState::Filled
        };
        Self {
            coordinate,
            icon,
            label,
            hotkey,
            badge_state,
            state,
            is_dragging_source: false,
            is_drag_over: false,
        }
    }
}

impl ddd::View for TileFaceView {}
