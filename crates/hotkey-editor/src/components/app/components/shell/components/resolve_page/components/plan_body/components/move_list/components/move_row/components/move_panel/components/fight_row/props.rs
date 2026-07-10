use crate::components::app::components::shell::components::resolve_page::logic::MoveView;
use dioxus::prelude::*;

/// The fighting-abilities row: the mover column beside the optional rival column, both
/// derived from the one move.
#[derive(Props, Clone, PartialEq)]
pub struct FightRowProps {
    pub move_view: MoveView,
}
