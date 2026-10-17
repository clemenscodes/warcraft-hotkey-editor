use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::shared::grid_editor::components::shared::hotkey_badge::HotkeyBadgeState;
use crate::components::app::components::shell::components::shared::grid_tile::GridTileState;
use crate::components::app::components::shell::components::shared::tile_face::TileFaceView;
use warcraft_keybinds::GridCoordinate;
use warcraft_keybinds::HotkeyToken;
use warcraft_keybinds::RenderedTile;

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
use super::model::PreviewGridModel;

pub(super) fn use_preview_grid(props: &PreviewGridModel) -> Vec<PreviewTile> {
    props.tiles.iter().map(PreviewTile::from).collect()
}
