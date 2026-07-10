use crate::components::app::components::shell::components::resolve_page::logic::MoveView;
use dioxus::prelude::*;

/// The mover's fighter column: the move whose mover it renders as a name button
/// stacked over an ability icon.
#[derive(Props, Clone, PartialEq)]
pub struct FightColumnProps {
    pub move_view: MoveView,
}
