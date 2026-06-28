use dioxus::prelude::*;
use warcraft_api::Race;
use warcraft_keybinds::{ColumnIndex, GridCoordinate, HotkeyToken, RowIndex};

use super::components::HotkeyBadgeState;
use super::state::GridTileState;

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
