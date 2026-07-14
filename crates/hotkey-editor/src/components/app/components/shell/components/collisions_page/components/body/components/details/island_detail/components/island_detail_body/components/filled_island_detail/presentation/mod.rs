use super::model::FilledIslandDetailModel;
use crate::components::app::components::shell::components::collisions_page::presentation::ConflictView;
use warcraft_keybinds::GridCoordinate;

pub struct FilledIslandDetailPresentation {
    pub(super) coordinate: GridCoordinate,
    pub(super) count: usize,
    pub(super) conflicts: Vec<ConflictView>,
}

impl From<&FilledIslandDetailModel> for FilledIslandDetailPresentation {
    fn from(model: &FilledIslandDetailModel) -> Self {
        let island = &model.island;
        let coordinate = island.coordinate();
        let count = island.collision_count();
        let conflicts = island.conflicts().to_vec();
        Self {
            coordinate,
            count,
            conflicts,
        }
    }
}

impl ddd::Presentation for FilledIslandDetailPresentation {
    type Model = FilledIslandDetailModel;
}
