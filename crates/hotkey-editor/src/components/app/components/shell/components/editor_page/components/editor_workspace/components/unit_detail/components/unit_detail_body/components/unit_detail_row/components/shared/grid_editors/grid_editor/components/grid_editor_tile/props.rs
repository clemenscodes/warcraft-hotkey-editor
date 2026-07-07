use super::components::tile_face::TileFaceProps;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::grid_editor::components::shared::hotkey_badge::HotkeyBadgeState;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::grid_editor::components::headed_grid::components::grid::components::grid_tile::GridTileState;
use dioxus::prelude::*;
use warcraft_api::Race;
use warcraft_keybinds::{ColumnIndex, GridCoordinate, HotkeyToken, RenderedTile, RowIndex};

/// The interactive editor tile's props: the `TileFace` painter's visual fields plus the
/// editor's interaction — focus, drag state, and the event handlers. This is the only
/// tile that expects a hotkey. Its address is the domain `GridCoordinate`; its hotkey is
/// the domain `HotkeyToken`.
#[derive(Props, Clone, PartialEq)]
pub struct GridEditorTileProps {
    #[props(default = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Zero))]
    pub coordinate: GridCoordinate,
    #[props(default = Race::Neutral)]
    pub race: Race,
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

impl From<&GridEditorTileProps> for TileFaceProps {
    /// The painter's slice of the editor tile: the visual fields, none of the interaction.
    fn from(props: &GridEditorTileProps) -> Self {
        let coordinate = props.coordinate;
        let race = props.race;
        let icon = props.icon.clone();
        let label = props.label.clone();
        let hotkey = props.hotkey;
        let badge_state = props.badge_state;
        let state = props.state;
        Self {
            coordinate,
            race,
            icon,
            label,
            hotkey,
            badge_state,
            state,
        }
    }
}

impl From<&RenderedTile> for GridEditorTileProps {
    /// The read-only adaptation from a domain tile: the paint comes from `TileFaceProps`,
    /// and the editor overlays behavior on top. Here every handler stays at its default
    /// (`render.rs` wires them); only the two interaction flags the domain decides —
    /// focusability and draggability — are read from the rendered tile.
    fn from(rendered: &RenderedTile) -> Self {
        let face = TileFaceProps::from(rendered);
        let TileFaceProps {
            coordinate,
            race,
            icon,
            label,
            hotkey,
            badge_state,
            state,
        } = face;
        let is_focusable = rendered.occupant().is_some();
        let draggable = rendered.draggable();
        Self {
            coordinate,
            race,
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
