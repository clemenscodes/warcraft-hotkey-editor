use super::view::UnitPositionConflictGridView;
use crate::components::app::components::shell::components::collisions_page::presentation::UnitPositionConflictView;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

#[derive(Props, Clone, PartialEq)]
pub struct UnitPositionConflictGridModel {
    pub conflicts: Vec<UnitPositionConflictView>,
    pub unit_id: WarcraftObjectId,
}

impl From<&UnitPositionConflictGridView> for UnitPositionConflictGridModel {
    fn from(view: &UnitPositionConflictGridView) -> Self {
        let UnitPositionConflictGridView { conflicts, unit_id } = view.clone();
        Self { conflicts, unit_id }
    }
}

impl ddd::Model for UnitPositionConflictGridModel {
    type View = UnitPositionConflictGridView;
}
