use super::view::UnitCardsSidebarView;
use crate::components::app::components::shell::components::collisions_page::presentation::CollisionUnitView;
use dioxus::prelude::*;

/// The unit sidebar: the clashing units to render as cards. Generic over the conflict
/// shape so the hotkey and unit-position kinds share one sidebar; the selected unit is
/// read from collision-selection context by the component, not threaded here.
#[derive(Props, Clone, PartialEq)]
pub struct UnitCardsSidebarModel<Conflict: Clone + PartialEq + 'static> {
    pub units: Vec<CollisionUnitView<Conflict>>,
}

impl<Conflict: Clone + PartialEq + 'static> From<&UnitCardsSidebarView<Conflict>>
    for UnitCardsSidebarModel<Conflict>
{
    fn from(view: &UnitCardsSidebarView<Conflict>) -> Self {
        let UnitCardsSidebarView { units } = view.clone();
        Self { units }
    }
}

impl<Conflict: Clone + PartialEq + 'static> ddd::Model for UnitCardsSidebarModel<Conflict> {
    type View = UnitCardsSidebarView<Conflict>;
}
