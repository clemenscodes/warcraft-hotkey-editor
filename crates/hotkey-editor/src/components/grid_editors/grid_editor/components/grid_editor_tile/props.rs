use super::components::hotkey_badge::HotkeyBadgeState;
use crate::components::grid_editors::grid_editor::components::headed_grid::components::grid::components::grid_tile::{
    GridTileProps, GridTileState,
};
use crate::model::icons::IconUrl;
use dioxus::prelude::*;
use warcraft_api::Race;
use warcraft_keybinds::{ColumnIndex, GridCoordinate, HotkeyToken, RenderedTile, RowIndex};

/// The interactive editor tile. It wraps the inert base `GridTile` and layers the
/// hotkey badge and every editor concern on top: focus, drag state, and the event
/// handlers. This is the only tile that expects a hotkey. Its address is the
/// domain `GridCoordinate`; its hotkey is the domain `HotkeyToken`.
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

impl From<&GridEditorTileProps> for GridTileProps {
    /// The inert base tile: only the presentational fields, none of the editor's
    /// interaction.
    fn from(props: &GridEditorTileProps) -> Self {
        let coordinate = props.coordinate;
        let race = props.race;
        let icon = props.icon.clone();
        let label = props.label.clone();
        let state = props.state;
        Self {
            coordinate,
            race,
            icon,
            label,
            state,
        }
    }
}

impl From<&RenderedTile> for GridEditorTileProps {
    /// The one adaptation the UI performs on a domain tile: a raw icon path becomes
    /// an asset URL and the domain flags pick the widget's visual enums. No
    /// decision is made here, and no interaction is wired: every handler and drag
    /// flag stays at its default. The editor overlays those on top; a read-only
    /// consumer like the templates preview uses this as-is.
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
        let is_focusable = rendered.occupant().is_some();
        let draggable = rendered.draggable();
        Self {
            coordinate,
            race: Race::Neutral,
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
