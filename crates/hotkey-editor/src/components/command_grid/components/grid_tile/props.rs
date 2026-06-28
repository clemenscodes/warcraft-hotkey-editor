use dioxus::prelude::*;
use warcraft_api::Race;

use super::components::HotkeyBadgeState;
use super::state::GridTileState;

/// A purely presentational command tile. It knows how to draw an icon, a label,
/// a hotkey badge, and the look of each visual state. It owns no domain logic:
/// the grid decides which ability sits here, resolves the state, and supplies the
/// event handlers. The tile only renders and forwards events.
#[derive(Props, Clone, PartialEq)]
pub struct GridTileProps {
    /// The owning unit's race, themeing the hover/selected accent. Forwarded from
    /// the grid (one race per grid), set as a `data-race` attribute so the tile's
    /// own CSS resolves the accent color, instead of inheriting it ambiently.
    #[props(default = Race::Neutral)]
    pub race: Race,

    /// Ability icon URL, drawn filling the tile when present.
    #[props(default)]
    pub icon: Option<String>,
    /// Shown centered when the tile is focusable and has no icon.
    #[props(default)]
    pub label: String,
    /// The hotkey letter; renders a badge when present.
    #[props(default)]
    pub hotkey: Option<String>,
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

    /// The grid decorates the tile with its own positional data attributes
    /// (`data-grid-row`/`-col`/`-section`) for pointer hit-testing. The tile
    /// stays oblivious to where it sits; it just spreads what it is given.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}
