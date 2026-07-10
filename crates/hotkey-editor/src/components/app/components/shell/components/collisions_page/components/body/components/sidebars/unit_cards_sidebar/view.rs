use crate::components::app::components::shell::components::collisions_page::logic::CollisionUnitView;

/// The published `View` contract mirroring [`UnitCardsSidebarProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct UnitCardsSidebarView<Conflict: Clone + PartialEq + 'static> {
    pub units: Vec<CollisionUnitView<Conflict>>,
}

impl<Conflict: Clone + PartialEq + 'static> ddd::View for UnitCardsSidebarView<Conflict> {}
