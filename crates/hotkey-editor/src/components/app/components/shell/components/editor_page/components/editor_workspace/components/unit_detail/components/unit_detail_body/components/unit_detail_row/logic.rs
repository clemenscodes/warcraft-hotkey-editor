use super::data::HEADING;
use super::props::UnitDetailRowProps;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::grid_editor::components::headed_grid::components::grid_heading::GridHeadingProps;

impl From<&UnitDetailRowProps> for GridHeadingProps {
    fn from(_props: &UnitDetailRowProps) -> Self {
        let heading = HEADING;
        Self { heading }
    }
}
