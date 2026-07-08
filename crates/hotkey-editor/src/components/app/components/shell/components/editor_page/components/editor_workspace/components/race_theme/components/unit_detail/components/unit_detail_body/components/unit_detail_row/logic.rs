use super::data::HEADING;
use super::props::UnitDetailRowProps;
use crate::components::app::components::shell::components::shared::grid_heading::GridHeadingProps;

impl From<&UnitDetailRowProps> for GridHeadingProps {
    fn from(_props: &UnitDetailRowProps) -> Self {
        let heading = HEADING;
        Self { heading }
    }
}
