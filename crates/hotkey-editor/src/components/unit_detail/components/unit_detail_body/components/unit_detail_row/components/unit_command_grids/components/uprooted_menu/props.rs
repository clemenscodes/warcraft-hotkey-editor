use crate::components::grid_editors::grid_editor::GridEditorConfig;
use dioxus::prelude::*;

/// The unit's optional uprooted-form menu grid; renders nothing when absent.
#[derive(Props, Clone, PartialEq)]
pub struct UprootedMenuProps {
    pub config: Option<GridEditorConfig>,
}
