use dioxus::prelude::*;
use warcraft_keybinds::HotkeyToken;

use super::super::super::GridTileProps;
use super::super::super::GridTileState;
use super::super::super::TileChrome;
use super::super::hotkey_badge::HotkeyBadgeState;
use super::state::EmptyTileState;

/// An empty command slot: the position's hotkey badge and the shared tile chrome.
/// During a drag it can become a drop target (or a blocked one).
#[derive(Props, Clone, PartialEq)]
pub struct EmptyTileProps {
    pub chrome: TileChrome,
    pub state: EmptyTileState,
    /// `"true"` when this empty slot is the active drop-target candidate, as a
    /// `data-drop-target` hook for the position-picker styling (the tile's own
    /// look comes from `state`).
    pub drop_target: &'static str,
    pub hotkey: HotkeyToken,
    pub badge_state: HotkeyBadgeState,
}

impl From<&GridTileProps> for EmptyTileProps {
    fn from(props: &GridTileProps) -> Self {
        let state = match props.state {
            GridTileState::DropTarget => EmptyTileState::DropTarget,
            GridTileState::BlockedDropTarget => EmptyTileState::BlockedDropTarget,
            _ => EmptyTileState::Empty,
        };
        let drop_target = if matches!(props.state, GridTileState::DropTarget) {
            "true"
        } else {
            "false"
        };
        let chrome = TileChrome::from(props);
        let hotkey = props.hotkey;
        let badge_state = props.badge_state;
        Self {
            chrome,
            state,
            drop_target,
            hotkey,
            badge_state,
        }
    }
}
