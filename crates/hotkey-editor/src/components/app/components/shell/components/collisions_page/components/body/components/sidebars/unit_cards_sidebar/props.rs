use crate::components::app::components::shell::components::collisions_page::logic::CollisionUnitView;
use dioxus::prelude::*;

/// The unit sidebar: the clashing units and the selected key it drives. Generic
/// over the conflict shape so the hotkey and unit-position kinds share one sidebar.
#[derive(Props, Clone, PartialEq)]
pub struct UnitCardsSidebarProps<Conflict: Clone + PartialEq + 'static> {
    pub units: Vec<CollisionUnitView<Conflict>>,
    pub selected_unit: Signal<Option<String>>,
}
