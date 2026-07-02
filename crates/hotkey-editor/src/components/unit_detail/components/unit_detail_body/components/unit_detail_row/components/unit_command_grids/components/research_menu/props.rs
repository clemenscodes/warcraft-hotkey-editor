use crate::components::grid_editors::grid_editor::GridEditorConfig;
use dioxus::prelude::*;

/// The unit's optional research menu grid; renders nothing when absent.
#[derive(Props, Clone, PartialEq)]
pub struct ResearchMenuProps {
    pub config: Option<GridEditorConfig>,
}
