use super::super::super::props::FilledTileProps;
use super::super::super::state::FilledTileKind;
use dioxus::prelude::*;

/// The ability fill draws only when the occupant is an ordinary ability (a selected
/// tile keeps the ability background too). A command occupant draws `CommandFill`
/// instead, so this fill early-returns.
#[derive(Props, Clone, PartialEq)]
pub struct AbilityFillProps {
    pub active: bool,
}

impl From<&FilledTileProps> for AbilityFillProps {
    fn from(props: &FilledTileProps) -> Self {
        let active = matches!(props.kind, FilledTileKind::Ability);
        Self { active }
    }
}
