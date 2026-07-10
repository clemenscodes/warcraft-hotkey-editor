use super::view::FightRowView;
use crate::components::app::components::shell::components::resolve_page::logic::MoveView;
use dioxus::prelude::*;

/// The fighting-abilities row: the mover column beside the optional rival column, both
/// derived from the one move.
#[derive(Props, Clone, PartialEq)]
pub struct FightRowProps {
    pub move_view: MoveView,
}

impl From<&FightRowView> for FightRowProps {
    fn from(view: &FightRowView) -> Self {
        let FightRowView { move_view } = view.clone();
        Self { move_view }
    }
}

impl ddd::Props for FightRowProps {
    type View = FightRowView;
}
