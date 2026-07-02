use super::super::super::GridTileProps;
use super::super::super::GridTileState;
use super::super::super::TileChrome;
use super::state::FilledTileState;
use dioxus::prelude::*;

/// An occupied command tile: the ability/command icon (or its text fallback) and
/// the shared tile chrome.
#[derive(Props, Clone, PartialEq)]
pub struct FilledTileProps {
    pub chrome: TileChrome,
    pub state: FilledTileState,
    /// `"true"` when this is the selected slot, as a `data-selected` hook for the
    /// keyboard-focus service (the tile's own selected look comes from `state`).
    pub selected: &'static str,
    pub icon: Option<String>,
    pub label: String,
}

/// Marks a slot that is not occupied, so it cannot become a `FilledTile`.
pub struct NotFilled;

impl TryFrom<&GridTileProps> for FilledTileProps {
    type Error = NotFilled;

    fn try_from(props: &GridTileProps) -> Result<Self, Self::Error> {
        let state = match props.state {
            GridTileState::Filled => FilledTileState::Filled,
            GridTileState::Command => FilledTileState::Command,
            GridTileState::Selected => FilledTileState::Selected,
            GridTileState::Empty
            | GridTileState::DropTarget
            | GridTileState::BlockedDropTarget
            | GridTileState::Highlighted => {
                return Err(NotFilled);
            }
        };
        let selected = if matches!(props.state, GridTileState::Selected) {
            "true"
        } else {
            "false"
        };
        let chrome = TileChrome::from(props);
        let icon = props.icon.clone();
        let label = props.label.clone();
        Ok(Self {
            chrome,
            state,
            selected,
            icon,
            label,
        })
    }
}
