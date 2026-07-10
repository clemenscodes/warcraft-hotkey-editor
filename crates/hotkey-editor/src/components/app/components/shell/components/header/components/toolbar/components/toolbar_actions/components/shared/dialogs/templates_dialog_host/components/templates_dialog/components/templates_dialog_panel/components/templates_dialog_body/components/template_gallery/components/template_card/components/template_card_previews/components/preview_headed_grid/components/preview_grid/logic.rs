use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::grid_editor::components::shared::hotkey_badge::HotkeyBadgeState;
use crate::components::app::components::shell::components::shared::grid_tile::GridTileState;
use crate::components::app::components::shell::components::shared::tile_face::TileFaceProps;
use warcraft_keybinds::{GridCoordinate, HotkeyToken, RenderedTile};

/// One preview tile's painted values: the read-only slice of a `TileFace` a preview
/// draws — its address, icon, label, hotkey, and the badge/tile states. Adapted from a
/// resolved domain tile through the shared painter's own `From<&RenderedTile>`, so the
/// preview draws exactly the tiles the editor does, without re-deriving the adaptation.
pub(super) struct PreviewTile {
    pub(super) coordinate: GridCoordinate,
    pub(super) icon: Option<String>,
    pub(super) label: String,
    pub(super) hotkey: HotkeyToken,
    pub(super) badge_state: HotkeyBadgeState,
    pub(super) state: GridTileState,
}

impl From<&RenderedTile> for PreviewTile {
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
        Self {
            coordinate,
            icon,
            label,
            hotkey,
            badge_state,
            state,
        }
    }
}
