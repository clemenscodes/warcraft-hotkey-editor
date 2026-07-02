use crate::components::grid_editors::grid_editor::GridEditorConfig;
use dioxus::prelude::*;

/// The unit's optional build menu grid; renders nothing when the unit has none.
#[derive(Props, Clone, PartialEq)]
pub struct BuildMenuProps {
    pub config: Option<GridEditorConfig>,
}
