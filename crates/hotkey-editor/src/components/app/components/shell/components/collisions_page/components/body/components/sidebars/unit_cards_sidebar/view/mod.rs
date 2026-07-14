use crate::components::app::components::shell::components::collisions_page::presentation::CollisionUnitView;

#[derive(Clone, PartialEq)]
pub struct UnitCardsSidebarView<Conflict: Clone + PartialEq + 'static> {
    pub units: Vec<CollisionUnitView<Conflict>>,
}

impl<Conflict: Clone + PartialEq + 'static> ddd::View for UnitCardsSidebarView<Conflict> {}
