use super::model::FilledUnitPositionDetailModel;
use crate::components::app::components::shell::components::collisions_page::presentation::UnitIconView;
use crate::components::app::components::shell::components::collisions_page::presentation::UnitPositionConflictView;
use warcraft_api::WarcraftObjectId;

/// The populated position-collision detail pane's presentation: the selected unit, its
/// collision count, the owning unit id, and the position conflicts. Built purely from
/// the model — a shaping leaf, no effects.
pub struct FilledUnitPositionDetailPresentation {
    pub(super) unit: UnitIconView,
    pub(super) count: usize,
    pub(super) unit_id: WarcraftObjectId,
    pub(super) conflicts: Vec<UnitPositionConflictView>,
}

impl From<&FilledUnitPositionDetailModel> for FilledUnitPositionDetailPresentation {
    fn from(model: &FilledUnitPositionDetailModel) -> Self {
        let unit_view = &model.unit_view;
        let unit = unit_view.unit().clone();
        let count = unit_view.collision_count();
        let unit_id = unit.unit_id();
        let conflicts = unit_view.conflicts().to_vec();
        Self {
            unit,
            count,
            unit_id,
            conflicts,
        }
    }
}

impl ddd::Presentation for FilledUnitPositionDetailPresentation {
    type Model = FilledUnitPositionDetailModel;
}
