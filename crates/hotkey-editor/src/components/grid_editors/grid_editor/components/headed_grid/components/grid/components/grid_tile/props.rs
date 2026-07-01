use super::components::hotkey_badge::HotkeyBadgeState;
use super::state::GridTileState;
use crate::model::icons::IconUrl;
use dioxus::prelude::*;
use warcraft_api::Race;
use warcraft_keybinds::{ColumnIndex, GridCoordinate, HotkeyToken, RenderedTile, RowIndex};

/// Everything a command tile needs to render and report events, built by the grid
/// engine and passed as one value. The tile is purely presentational: it draws an
/// icon, a label, a hotkey badge, and the look of each state, and forwards events.
/// Its address is the domain `GridCoordinate`; its hotkey is the domain
/// `HotkeyToken`.
#[derive(Props, Clone, PartialEq)]
pub struct GridTileProps {
    /// Where the tile sits, emitted as `data-grid-row`/`-col` for pointer
    /// hit-testing.
    #[props(default = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Zero))]
    pub coordinate: GridCoordinate,
    /// The owning unit's race, theming the hover/selected accent via `data-race`.
    #[props(default = Race::Neutral)]
    pub race: Race,
    /// Ability icon URL, drawn filling the tile when present.
    #[props(default)]
    pub icon: Option<String>,
    /// Shown centered when the tile is focusable and has no icon.
    #[props(default)]
    pub label: String,
    /// The hotkey; every tile always has one, shown as its badge.
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

impl From<&RenderedTile> for GridTileProps {
    /// The one adaptation the UI performs on a domain tile: a raw icon path
    /// becomes an asset URL and the domain flags pick the widget's visual enums.
    /// No decision is made here, and no interaction is wired: every handler and
    /// drag flag stays at its default. The editor overlays those on top; a
    /// read-only consumer like the templates preview uses this as-is.
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
