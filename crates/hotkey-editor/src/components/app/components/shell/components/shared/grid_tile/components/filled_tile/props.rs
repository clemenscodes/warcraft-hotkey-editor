use super::super::super::GridTileProps;
use super::super::super::GridTileState;
use super::super::super::TileChrome;
use super::state::FilledTileKind;
use dioxus::prelude::*;

/// An occupied command tile: the ability/command icon (or its text fallback) and the
/// shared tile chrome. Its background is chosen by `kind` (an `AbilityFill` or a
/// `CommandFill` child), and `selected` mounts the `SelectionRing` child — neither is
/// a class swap, so the tile root stays one mounted element.
#[derive(Props, Clone, PartialEq)]
pub struct FilledTileProps {
    pub chrome: TileChrome,
    pub kind: FilledTileKind,
    /// Whether this is the selected slot. When true the tile mounts `SelectionRing`,
    /// and its root turns gold and glows via `:has(.selection-ring)`.
    pub selected: bool,
    pub icon: Option<String>,
    pub label: String,
}

/// Marks a slot that is not occupied, so it cannot become a `FilledTile`.
pub struct NotFilled;

impl TryFrom<&GridTileProps> for FilledTileProps {
    type Error = NotFilled;

    fn try_from(props: &GridTileProps) -> Result<Self, Self::Error> {
        let kind = match props.state {
            GridTileState::Filled | GridTileState::Selected => FilledTileKind::Ability,
            GridTileState::Command => FilledTileKind::Command,
            GridTileState::Empty
            | GridTileState::DropTarget
            | GridTileState::BlockedDropTarget
            | GridTileState::Highlighted => {
                return Err(NotFilled);
            }
        };
        let selected = matches!(props.state, GridTileState::Selected);
        let chrome = TileChrome::from(props);
        let icon = props.icon.clone();
        let label = props.label.clone();
        Ok(Self {
            chrome,
            kind,
            selected,
            icon,
            label,
        })
    }
}
