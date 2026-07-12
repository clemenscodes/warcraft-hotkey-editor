use super::view::FightRowView;
use crate::components::app::components::shell::components::resolve_page::presentation::MoveView;
use dioxus::prelude::*;

/// The fighting-abilities row: the mover column beside the optional rival column, both
/// derived from the one move.
#[derive(Props, Clone, PartialEq)]
pub struct FightRowModel {
    pub move_view: MoveView,
}

impl From<&FightRowView> for FightRowModel {
    fn from(view: &FightRowView) -> Self {
        let FightRowView { move_view } = view.clone();
        Self { move_view }
    }
}

impl ddd::Model for FightRowModel {
    type View = FightRowView;
}
