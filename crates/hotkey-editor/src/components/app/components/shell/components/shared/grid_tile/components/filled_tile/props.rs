use super::super::super::GridTileProps;
use super::super::super::GridTileState;
use super::state::FilledTileKind;
use dioxus::prelude::*;

/// An occupied command tile: the ability/command icon (or its text fallback). Its
/// background is chosen by `kind` (an `AbilityFill` or a `CommandFill` child),
/// `selected` mounts the `SelectionRing` child, `is_dragging_source` the
/// `DraggingSourceGhost`, and `is_drag_over` the `DragOverRing` — none is a class swap,
/// so the tile root stays one mounted element across every state.
#[derive(Props, Clone, PartialEq)]
pub struct FilledTileProps {
    pub kind: FilledTileKind,
    /// Whether this is the selected slot. When true the tile mounts `SelectionRing`,
    /// and its root turns gold and glows via `:has(.selection-ring)`.
    pub selected: bool,
    /// True while this tile is the lifted source of a drag: it mounts the
    /// `DraggingSourceGhost`, and its root turns into the dashed deep-blue ghost.
    pub is_dragging_source: bool,
    /// True while the drag cursor hovers this tile: it mounts the `DragOverRing`, and
    /// its root's border turns gold.
    pub is_drag_over: bool,
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
        let is_dragging_source = props.is_dragging_source;
        let is_drag_over = props.is_drag_over;
        let icon = props.icon.clone();
        let label = props.label.clone();
        Ok(Self {
            kind,
            selected,
            is_dragging_source,
            is_drag_over,
            icon,
            label,
        })
    }
}
