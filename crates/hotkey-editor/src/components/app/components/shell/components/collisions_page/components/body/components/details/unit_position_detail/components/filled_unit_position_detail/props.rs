use super::view::FilledUnitPositionDetailView;
use crate::components::app::components::shell::components::collisions_page::logic::UnitPositionUnitView;
use dioxus::prelude::*;

/// The populated position-collision detail pane: the selected unit's view, whose header
/// and position-conflict cards this pane shapes and renders.
#[derive(Props, Clone, PartialEq)]
pub struct FilledUnitPositionDetailProps {
    pub unit_view: UnitPositionUnitView,
}

impl From<&FilledUnitPositionDetailView> for FilledUnitPositionDetailProps {
    fn from(view: &FilledUnitPositionDetailView) -> Self {
        let FilledUnitPositionDetailView { unit_view } = view.clone();
        Self { unit_view }
    }
}

impl ddd::Props for FilledUnitPositionDetailProps {
    type View = FilledUnitPositionDetailView;
}
