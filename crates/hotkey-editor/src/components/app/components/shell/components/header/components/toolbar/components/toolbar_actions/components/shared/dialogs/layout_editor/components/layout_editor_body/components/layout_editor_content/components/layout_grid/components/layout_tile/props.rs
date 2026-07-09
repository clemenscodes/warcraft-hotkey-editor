use super::state::LayoutTileState;
use crate::components::app::components::shell::components::shared::editable_keycap::{
    EditableKeycapProps, EditableKeycapState,
};
use dioxus::prelude::*;
use warcraft_keybinds::GridCoordinate;

/// One editable grid cell: its visual state, the letter it shows, its grid
/// address, and the drag/click handlers the editor wired for it.
#[derive(Props, Clone, PartialEq)]
pub struct LayoutTileProps {
    pub state: LayoutTileState,
    pub label: String,
    pub coordinate: GridCoordinate,
    pub ondragstart: EventHandler<Event<DragData>>,
    pub ondragend: EventHandler<Event<DragData>>,
    pub ondragover: EventHandler<Event<DragData>>,
    pub ondrop: EventHandler<Event<DragData>>,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&LayoutTileProps> for EditableKeycapProps {
    fn from(props: &LayoutTileProps) -> Self {
        let label = props.label.clone();
        let state = match props.state {
            LayoutTileState::Idle => EditableKeycapState::Idle,
            LayoutTileState::Editing => EditableKeycapState::Editing,
        };
        Self { label, state }
    }
}
