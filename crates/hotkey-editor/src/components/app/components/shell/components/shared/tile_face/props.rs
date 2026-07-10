use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::grid_editor::components::shared::hotkey_badge::HotkeyBadgeState;
use crate::components::app::components::shell::components::shared::grid_tile::GridTileState;
use crate::components::app::components::shell::components::shared::icons::IconUrl;
use dioxus::prelude::*;
use warcraft_keybinds::{ColumnIndex, GridCoordinate, HotkeyToken, RenderedTile, RowIndex};

/// The tile painter's props: only the resting visual of a command-grid slot — its
/// address, icon, label, hotkey, and the badge/tile states. No interaction: no
/// handlers, no drag flags, no focus. The editor's `GridEditorTile` Host carries
/// those and converts down into this; a read-only consumer (templates preview,
/// gallery) builds it straight from a `RenderedTile`.
#[derive(Props, Clone, PartialEq)]
pub struct TileFaceProps {
    #[props(default = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Zero))]
    pub coordinate: GridCoordinate,
    #[props(default)]
    pub icon: Option<String>,
    #[props(default)]
    pub label: String,
    /// The hotkey; every tile always has one, shown as its badge.
    pub hotkey: HotkeyToken,
    #[props(default)]
    pub badge_state: HotkeyBadgeState,
    #[props(default)]
    pub state: GridTileState,
    /// True while this tile is the lifted source of a drag. The editor Host sets it; the
    /// read-only consumers (templates preview, gallery) leave it false.
    #[props(default)]
    pub is_dragging_source: bool,
    /// True while the drag cursor hovers this tile. The editor Host sets it; the
    /// read-only consumers leave it false.
    #[props(default)]
    pub is_drag_over: bool,
}

impl From<&RenderedTile> for TileFaceProps {
    /// The one adaptation the UI performs on a domain tile: a raw icon path becomes
    /// an asset URL and the domain flags pick the widget's visual enums. No
    /// decision is made here — this is pure paint, which the editor Host wraps with
    /// behavior and the read-only consumers use as-is.
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
