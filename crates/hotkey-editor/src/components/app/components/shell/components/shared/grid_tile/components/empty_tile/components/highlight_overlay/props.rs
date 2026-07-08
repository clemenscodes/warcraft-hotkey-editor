use super::super::super::props::EmptyTileProps;
use super::super::super::state::EmptyTileState;
use dioxus::prelude::*;

/// Mounts only on the one coordinate a mini grid marks; every other empty slot
/// early-returns.
#[derive(Props, Clone, PartialEq)]
pub struct HighlightOverlayProps {
    pub active: bool,
}

impl From<&EmptyTileProps> for HighlightOverlayProps {
    fn from(props: &EmptyTileProps) -> Self {
        let active = matches!(props.state, EmptyTileState::Highlighted);
        Self { active }
    }
}
