use super::super::super::props::FilledTileProps;
use super::super::super::state::FilledTileKind;
use dioxus::prelude::*;

/// The command fill draws only when the occupant is a built-in command; every other
/// occupant draws `AbilityFill`, so this fill early-returns.
#[derive(Props, Clone, PartialEq)]
pub struct CommandFillProps {
    pub active: bool,
}

impl From<&FilledTileProps> for CommandFillProps {
    fn from(props: &FilledTileProps) -> Self {
        let active = matches!(props.kind, FilledTileKind::Command);
        Self { active }
    }
}
