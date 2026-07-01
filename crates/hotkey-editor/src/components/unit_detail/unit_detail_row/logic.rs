use super::props::UnitDetailRowProps;
use crate::components::grid_editors::grid_editor::components::headed_grid::components::grid_heading::GridHeadingProps;

impl From<&UnitDetailRowProps> for GridHeadingProps {
    fn from(_props: &UnitDetailRowProps) -> Self {
        let heading = "Hotkey override";
        Self { heading }
    }
}
