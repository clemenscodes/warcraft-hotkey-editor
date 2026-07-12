use super::view::FightColumnView;
use crate::components::app::components::shell::components::resolve_page::presentation::MoveView;
use dioxus::prelude::*;

/// The mover's fighter column: the move whose mover it renders as a name button
/// stacked over an ability icon.
#[derive(Props, Clone, PartialEq)]
pub struct FightColumnModel {
    pub move_view: MoveView,
}

impl From<&FightColumnView> for FightColumnModel {
    fn from(view: &FightColumnView) -> Self {
        let FightColumnView { move_view } = view.clone();
        Self { move_view }
    }
}

impl ddd::Model for FightColumnModel {
    type View = FightColumnView;
}
