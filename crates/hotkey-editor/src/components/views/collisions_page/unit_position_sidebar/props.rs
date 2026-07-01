use super::super::UnitPositionUnitView;
use dioxus::prelude::*;

/// The unit sidebar: the clashing units and the selected key it drives.
#[derive(Props, Clone, PartialEq)]
pub struct UnitPositionSidebarProps {
    pub units: Vec<UnitPositionUnitView>,
    pub selected_unit: Signal<Option<String>>,
}
