use warcraft_keybinds::{GridCoordinate, HotkeyToken};

use crate::components::grid_editors::grid_editor::{GridTileState, HotkeyBadgeState};

/// The presentational flags a tile carries beyond its visual state: whether the
/// player may drag it, whether it holds a built-in command (for follower
/// styling), and whether it is passive (for the follower badge).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct GridTileFlags {
    draggable: bool,
    is_command: bool,
    is_passive: bool,
}

impl GridTileFlags {
    pub fn new(draggable: bool, is_command: bool, is_passive: bool) -> Self {
        Self {
            draggable,
            is_command,
            is_passive,
        }
    }

    pub fn draggable(&self) -> bool {
        self.draggable
    }

    pub fn is_command(&self) -> bool {
        self.is_command
    }

    pub fn is_passive(&self) -> bool {
        self.is_passive
    }
}

/// The fully resolved visual for one tile. The domain wrapper produces these (one
/// per tile) and hands them to the generic grid, which knows how to render and how
/// to build the drag follower from them but carries no domain logic. The base
/// `state` never carries a drag state: the grid overlays those from its own drag
/// mechanics. The tile address is the domain `GridCoordinate`.
#[derive(Clone, PartialEq, Debug)]
pub struct GridTileView {
    coordinate: GridCoordinate,
    icon: Option<String>,
    label: String,
    hotkey: HotkeyToken,
    badge_state: HotkeyBadgeState,
    state: GridTileState,
    flags: GridTileFlags,
}

impl GridTileView {
    pub fn new(
        coordinate: GridCoordinate,
        icon: Option<String>,
        label: String,
        hotkey: HotkeyToken,
        badge_state: HotkeyBadgeState,
        state: GridTileState,
        flags: GridTileFlags,
    ) -> Self {
        Self {
            coordinate,
            icon,
            label,
            hotkey,
            badge_state,
            state,
            flags,
        }
    }

    pub fn coordinate(&self) -> GridCoordinate {
        self.coordinate
    }

    pub fn column(&self) -> u8 {
        u8::from(self.coordinate.column())
    }

    pub fn row(&self) -> u8 {
        u8::from(self.coordinate.row())
    }

    pub fn icon(&self) -> Option<&str> {
        self.icon.as_deref()
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn hotkey(&self) -> HotkeyToken {
        self.hotkey
    }

    pub fn badge_state(&self) -> HotkeyBadgeState {
        self.badge_state
    }

    pub fn state(&self) -> GridTileState {
        self.state
    }

    pub fn draggable(&self) -> bool {
        self.flags.draggable()
    }

    pub fn is_command(&self) -> bool {
        self.flags.is_command()
    }

    pub fn is_passive(&self) -> bool {
        self.flags.is_passive()
    }
}
