use super::model::FilledHotkeyUnitDetailModel;
use crate::components::app::components::shell::components::collisions_page::presentation::HotkeyConflictView;
use crate::components::app::components::shell::components::collisions_page::presentation::UnitIconView;
use warcraft_api::WarcraftObjectId;

pub struct FilledHotkeyUnitDetailPresentation {
    pub(super) unit: UnitIconView,
    pub(super) count: usize,
    pub(super) unit_id: WarcraftObjectId,
    pub(super) conflicts: Vec<HotkeyConflictView>,
}

impl From<&FilledHotkeyUnitDetailModel> for FilledHotkeyUnitDetailPresentation {
    fn from(model: &FilledHotkeyUnitDetailModel) -> Self {
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

impl ddd::Presentation for FilledHotkeyUnitDetailPresentation {
    type Model = FilledHotkeyUnitDetailModel;
}
