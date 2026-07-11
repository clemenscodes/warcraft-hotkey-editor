use super::view::UnitPositionDetailHeaderView;
use crate::components::app::components::shell::components::collisions_page::presentation::UnitIconView;
use dioxus::prelude::*;

/// The position-collision detail pane header: the selected unit and its collision count.
/// The header builds the unit button and the text meta column from the unit view.
#[derive(Props, Clone, PartialEq)]
pub struct UnitPositionDetailHeaderModel {
    pub unit: UnitIconView,
    pub count: usize,
}

impl From<&UnitPositionDetailHeaderView> for UnitPositionDetailHeaderModel {
    fn from(view: &UnitPositionDetailHeaderView) -> Self {
        let UnitPositionDetailHeaderView { unit, count } = view.clone();
        Self { unit, count }
    }
}

impl ddd::Model for UnitPositionDetailHeaderModel {
    type View = UnitPositionDetailHeaderView;
}
